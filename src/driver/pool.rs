use super::*;
use crate::process::*;

use rand::{prelude::*, seq::SliceRandom};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

pub struct ClientPool {
    readers: Vec<Client>,
    writers: Vec<Client>,
}

impl ClientPool {
    pub async fn new<T: IntoClientRequest + Unpin + Clone>(
        config: PoolConfig<T>,
    ) -> Result<Self, ClientError> {
        let mut readers = Vec::with_capacity(config.per_read_endpoint);
        let mut writers = Vec::with_capacity(config.per_write_endpoint);

        if let Some(w) = config.write_req {
            for _ in (0..config.per_write_endpoint).into_iter() {
                writers.push(Client::new(w.clone(), config.timeout_ms).await?)
            }
        }
        if let Some(r) = config.read_req {
            for _ in (0..config.per_read_endpoint).into_iter() {
                readers.push(Client::new(r.clone(), config.timeout_ms).await?)
            }
        }

        Ok(Self { readers, writers })
    }

    pub async fn execute(&self, query: Traversal) -> Result<ClientResponse, ClientError> {
        let mut rng = thread_rng();
        let client = if query.is_mutating() {
            self.writers
                .choose(&mut rng)
                .ok_or(ClientError::NoClients)?
        } else {
            self.readers
                .choose(&mut rng)
                .ok_or(ClientError::NoClients)?
        };

        client.execute(query).await
    }
}

pub struct PoolConfig<T> {
    pub read_req: Option<T>,
    pub write_req: Option<T>,
    pub per_write_endpoint: usize,
    pub per_read_endpoint: usize,
    pub timeout_ms: u128,
}

impl<T: Clone> PoolConfig<T> {
    pub fn builder() -> Self {
        PoolConfig {
            read_req: None,
            write_req: None,
            per_write_endpoint: 0,
            per_read_endpoint: 0,
            timeout_ms: 30000,
        }
    }

    pub fn single_endpoint(&mut self, url: T) -> &mut Self {
        self.read_req = Some(url.clone());
        self.write_req = Some(url);
        self
    }

    pub fn read_endpoint(&mut self, url: T) -> &mut Self {
        self.read_req = Some(url);
        self
    }

    pub fn write_endpoint(&mut self, url: T) -> &mut Self {
        self.write_req = Some(url);
        self
    }

    pub fn write_clients(&mut self, n: usize) -> &mut Self {
        self.per_write_endpoint = n;
        self
    }

    pub fn read_clients(&mut self, n: usize) -> &mut Self {
        self.per_read_endpoint = n;
        self
    }

    pub fn timeout(&mut self, ms: u128) -> &mut Self {
        self.timeout_ms = ms;
        self
    }
}
