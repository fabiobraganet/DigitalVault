// src/client.rs
use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MinioClient {
    pub client: Arc<Mutex<Client>>,
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
}

impl MinioClient {
    pub fn new(endpoint: &str, access_key: &str, secret_key: &str) -> Self {
        let client = ClientBuilder::new()
            .http1_only()
            .build()
            .expect("Failed to build HTTP client");

        MinioClient {
            client: Arc::new(Mutex::new(client)),
            endpoint: endpoint.to_string(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }
}
