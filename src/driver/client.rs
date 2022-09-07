use futures::{stream::StreamExt, SinkExt};
use serde::{de, Serialize};
use serde_json::{self, from_slice, from_value, to_vec, Value};
use std::collections::HashMap;
use thiserror::Error;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{self, Message},
};
use uuid::Uuid;

type OneshotItem = Result<Vec<Value>, ClientError>;
type MpscItem = (Bytecode, oneshot::Sender<OneshotItem>);
type PendingItem = (Vec<Value>, oneshot::Sender<OneshotItem>);

use crate::process::{
    traversal::{Bytecode, Traversal},
    GValue, Process,
};

use super::parser::*;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error connecting to db server: {0}")]
    ErrorConnecting(#[from] tungstenite::Error),
    #[error("error sending gremlin request: {0}")]
    NetworkError(tungstenite::Error),
    #[error("server response error ({0})")]
    ResponseError(usize),
    #[error("gremlin request exceeded timeout")]
    RequestTimeout,
    #[error("error sending bytecode to processor (main client may have been dropped)")]
    ExecutionError,
    #[error("main client closed")]
    ClientClosed,
}

#[derive(Debug)]
pub struct ClientResponse(Vec<Value>);

impl ClientResponse {
    pub fn parse<T: de::DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        from_value::<T>(Value::Array(self.0.into_iter().map(unroll).collect()))
    }
}

impl From<Vec<Value>> for ClientResponse {
    fn from(v: Vec<Value>) -> Self {
        ClientResponse(v)
    }
}

pub struct Client {
    tx: mpsc::UnboundedSender<EventType>,
    main: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GremlinRequest {
    request_id: GValue,
    op: &'static str,
    processor: &'static str,
    args: RequestArgs,
}

#[derive(Serialize, Debug)]
struct RequestArgs {
    gremlin: Process,
    aliases: RequestAliases,
}

#[derive(Serialize, Debug)]
struct RequestAliases {
    g: &'static str,
}

impl GremlinRequest {
    pub fn new(bytecode: Bytecode) -> (Uuid, Self) {
        let request_id = Uuid::new_v4();
        (
            request_id.clone(),
            GremlinRequest {
                request_id: request_id.into(),
                op: "bytecode",
                processor: "traversal",
                args: RequestArgs {
                    gremlin: Process::Bytecode(bytecode),
                    aliases: RequestAliases { g: "g" },
                },
            },
        )
    }
}

impl Into<Message> for GremlinRequest {
    fn into(self) -> Message {
        let mut data = b"!application/vnd.gremlin-v2.0+json".to_vec();
        data.append(&mut to_vec(&self).unwrap());
        #[cfg(test)]
        println!(
            "sending request:\n{}",
            String::from_utf8(data.clone()).unwrap()
        );
        Message::Binary(data)
    }
}

#[derive(Debug)]
enum EventType {
    Ws(GremlinResponse),
    Rx(MpscItem),
    Kill,
}

use EventType::*;

impl Client {
    pub async fn new(url: &str) -> Result<Self, ClientError> {
        #[cfg(test)]
        println!("client connecting to websocket url: {}", url);

        let (wss, _) = connect_async_tls_with_config(url, None, None).await?;

        let (mut sink, mut stream) = wss.split();

        let (tx, rx) = mpsc::unbounded_channel::<EventType>();

        let mut rx_stream = Box::pin(UnboundedReceiverStream::new(rx));

        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(res) = stream.next().await {
                if let Ok(Message::Binary(bin)) = res {
                    if let Ok(response) = from_slice::<GremlinResponse>(&bin[..]) {
                        tx_clone.send(Ws(response)).unwrap()
                    } else {
                        #[cfg(test)]
                        println!(
                            "received invalid data from gremlin server:\n{}",
                            String::from_utf8(bin).unwrap()
                        )
                    }
                } else {
                    #[cfg(test)]
                    println!("recieved invalid message type from websocket:\n{:?}", res)
                }
            }
        });

        spawn(async move {
            let mut pending: HashMap<Uuid, PendingItem> = HashMap::new();
            while let Some(val) = rx_stream.next().await {
                match val {
                    Ws(res) => match res.status.code {
                        200 | 204 => {
                            if let Some((mut data, os_sender)) = pending.remove(&res.request_id) {
                                res.result.data.map(|mut d| data.append(&mut d));
                                os_sender.send(Ok(data)).unwrap();
                            }
                        }
                        206 => {
                            pending
                                .get_mut(&res.request_id)
                                .map(|p| res.result.data.map(|mut d| p.0.append(&mut d)));
                        }
                        _ => {
                            if let Some((_, os_sender)) = pending.remove(&res.request_id) {
                                os_sender
                                    .send(Err(ClientError::ResponseError(res.status.code)))
                                    .unwrap();
                            }
                        }
                    },
                    Rx((bytecode, os_sender)) => {
                        let (request_id, request) = GremlinRequest::new(bytecode);
                        match sink.send(request.into()).await {
                            Ok(_) => {
                                pending.insert(request_id.clone(), (Vec::new(), os_sender));
                            }
                            Err(e) => os_sender.send(Err(ClientError::NetworkError(e))).unwrap(),
                        };
                    }
                    Kill => {
                        #[cfg(test)]
                        println!("kill signal received");
                        rx_stream.close();
                        sink.send(Message::Close(None)).await.unwrap();
                        for (_, (_, sender)) in pending.drain() {
                            match sender.send(Err(ClientError::ClientClosed)) {
                                _ => (),
                            };
                        }
                        break;
                    }
                }
            }
        });

        #[cfg(test)]
        println!("created main client");

        Ok(Client { tx, main: true })
    }

    pub async fn execute(&self, query: Traversal) -> Result<ClientResponse, ClientError> {
        let bytecode: Bytecode = query.into();

        #[cfg(test)]
        println!("sending bytecode for execution: {:?}", &bytecode);

        let (os_tx, os_rx) = oneshot::channel();
        if let Err(_) = self.tx.send(Rx((bytecode, os_tx))) {
            return Err(ClientError::ExecutionError);
        }
        os_rx
            .await
            .map_err(|_| ClientError::ExecutionError)?
            .map(|v| v.into())
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Client {
            tx: self.tx.clone(),
            main: false,
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if self.main {
            #[cfg(test)]
            println!("killing main client");

            self.tx.send(Kill).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::to_string_pretty;

    use super::GremlinRequest;
    #[test]
    fn test_request_serialization() {
        let mut bytecode = process::traversal::Bytecode::new();
        bytecode.add_step(vec!["addV".into(), "user".into()]);
        let request = GremlinRequest::new(bytecode);
        println!("{}", to_string_pretty(&request).unwrap())
    }
}
