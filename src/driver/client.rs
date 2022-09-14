use futures::{stream::StreamExt, SinkExt};
use serde_json::to_vec;
use thiserror::Error;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    time::interval,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{self, client::IntoClientRequest, Message},
};
use uuid::Uuid;

use std::{
    collections::{HashMap, VecDeque},
    str::from_utf8,
    time,
};

#[cfg(test)]
use serde_json::to_string_pretty;

use crate::process::{bytecode::Bytecode, Traversal};

use super::serialize::*;

type OneshotItem = Result<Vec<Vec<u8>>, ClientError>;
type MpscItem = (Bytecode, oneshot::Sender<OneshotItem>);
type PendingItem = (Vec<Vec<u8>>, oneshot::Sender<OneshotItem>);

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("error connecting to db server: {0}")]
    ErrorConnecting(#[from] tungstenite::Error),
    #[error("error sending gremlin request: {0}")]
    NetworkError(tungstenite::Error),
    #[error("server response error ({0})")]
    ResponseError(usize, String),
    #[error("gremlin request exceeded timeout")]
    RequestTimeout,
    #[error("error sending bytecode to processor (main client may have been dropped)")]
    ExecutionError,
    #[error("main client closed")]
    ClientClosed,
    #[error("no available clients")]
    NoClients,
}

#[derive(Debug)]
pub struct ClientResponse(pub Vec<Vec<u8>>);

impl From<Vec<Vec<u8>>> for ClientResponse {
    fn from(v: Vec<Vec<u8>>) -> Self {
        ClientResponse(v)
    }
}

pub struct Client {
    tx: mpsc::UnboundedSender<EventType>,
    main: bool,
}

impl Into<Message> for GremlinRequest {
    fn into(self) -> Message {
        let mut data = b"!application/vnd.gremlin-v2.0+json".to_vec();
        data.append(&mut to_vec(&self).unwrap());
        Message::Binary(data)
    }
}

#[derive(Debug)]
pub(crate) enum EventType {
    Ws(Vec<u8>),
    Rx(MpscItem),
    Kill,
    Timeouts,
}

use EventType::*;

impl Client {
    pub async fn new<T: IntoClientRequest + Unpin>(
        url: T,
        timeout_ms: u128,
    ) -> Result<Self, ClientError> {
        let (wss, _) = connect_async_tls_with_config(url, None, None).await?;

        let (mut sink, mut stream) = wss.split();

        let (tx, rx) = mpsc::unbounded_channel::<EventType>();

        let mut rx_stream = Box::pin(UnboundedReceiverStream::new(rx));

        let tx_clone = tx.clone();
        spawn(async move {
            while let Some(res) = stream.next().await {
                if let Ok(Message::Binary(bin)) = res {
                    if let Err(_) = tx_clone.send(Ws(bin)) {
                        break;
                    }
                }
            }
        });

        let tx_clone = tx.clone();
        spawn(async move {
            let mut interval = interval(time::Duration::from_millis(100));
            loop {
                interval.tick().await;
                if let Err(_) = tx_clone.send(Timeouts) {
                    break;
                }
            }
        });

        spawn(async move {
            let mut pending: HashMap<Uuid, PendingItem> = HashMap::new();
            let mut timeouts: VecDeque<(u128, Uuid)> = VecDeque::new();
            while let Some(val) = rx_stream.next().await {
                match val {
                    Ws(res) => {
                        let header = parse_response_header(&res);

                        #[cfg(test)]
                        if let Err(e) = &header {
                            println!(
                                "error parsing header: {}\n{}\n",
                                e,
                                from_utf8(&res).unwrap_or("invalid_utf8")
                            );
                            for (_, (_, sender)) in pending.drain() {
                                sender.send(Err(ClientError::NoClients)).unwrap();
                            }
                        }

                        if let Ok(h) = header {
                            #[cfg(test)]
                            if let None = &h.request_id {
                                println!(
                                    "no request id received:\n{}\n{}",
                                    to_string_pretty(&h).unwrap(),
                                    from_utf8(&res).unwrap_or("invalid_utf8")
                                );
                                for (_, (_, sender)) in pending.drain() {
                                    sender.send(Err(ClientError::NoClients)).unwrap();
                                }
                            }

                            if let Some(request_id) = h.request_id {
                                match h.status.code {
                                    200 | 204 => {
                                        if let Some((mut data, os_sender)) =
                                            pending.remove(&request_id)
                                        {
                                            data.push(res);
                                            match os_sender.send(Ok(data)) {
                                                _ => {}
                                            }
                                        }
                                    }
                                    206 => {
                                        if let Some(p) = pending.get_mut(&request_id) {
                                            p.0.push(res);
                                        }
                                    }
                                    x => {
                                        if let Some((_, os_sender)) = pending.remove(&request_id) {
                                            match os_sender.send(Err(ClientError::ResponseError(
                                                x as usize,
                                                from_utf8(&res)
                                                    .unwrap_or("invalid utf8")
                                                    .to_string(),
                                            ))) {
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Rx((bytecode, os_sender)) => {
                        let (request_id, request) = GremlinRequest::new(bytecode);
                        match sink.send(request.into()).await {
                            Ok(_) => {
                                pending.insert(request_id.clone(), (Vec::new(), os_sender));
                                timeouts.push_back((
                                    time::SystemTime::now()
                                        .duration_since(time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis()
                                        + timeout_ms,
                                    request_id,
                                ))
                            }
                            Err(e) => os_sender.send(Err(ClientError::NetworkError(e))).unwrap(),
                        };
                    }
                    Kill => {
                        rx_stream.close();
                        sink.send(Message::Close(None)).await.unwrap();
                        for (_, (_, sender)) in pending.drain() {
                            match sender.send(Err(ClientError::ClientClosed)) {
                                _ => (),
                            };
                        }
                        break;
                    }
                    Timeouts => {
                        let now = time::SystemTime::now()
                            .duration_since(time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();

                        loop {
                            if let Some((t, request_id)) = timeouts.pop_front() {
                                if t < now {
                                    if let Some((_, os_sender)) = pending.remove(&request_id) {
                                        match os_sender.send(Err(ClientError::RequestTimeout)) {
                                            _ => {}
                                        }
                                    }
                                } else {
                                    timeouts.push_front((t, request_id));
                                    break;
                                }
                            }
                        }
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
            self.tx.send(Kill).unwrap()
        }
    }
}
