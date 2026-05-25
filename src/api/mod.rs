use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use serde::de::DeserializeOwned;
use anyhow::{Result, anyhow};
use std::sync::{Arc, RwLock};
use crate::config::Config;

pub struct ApiClient {
    client: Client,
    config: Arc<RwLock<Config>>,
    hostname: String,
}

impl ApiClient {
    pub fn new(config: Arc<RwLock<Config>>, hostname: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_static("application/vnd.github.v3+json"));
        headers.insert("User-Agent", HeaderValue::from_static("gh-rs"));

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            config,
            hostname: hostname.to_string(),
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("https://api.{}/{}", self.hostname, path.trim_start_matches('/'))
        };

        let mut req = self.client.get(&url);

        if let Some(token) = self.config.read().unwrap().token_for_host(&self.hostname) {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            return Err(anyhow!("API request failed with status: {}", resp.status()));
        }

        resp.json::<T>().await.map_err(|e| anyhow!("Failed to parse response: {}", e))
    }

    pub async fn post<T: DeserializeOwned, B: serde::Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("https://api.{}/{}", self.hostname, path.trim_start_matches('/'))
        };

        let mut req = self.client.post(&url).json(body);

        if let Some(token) = self.config.read().unwrap().token_for_host(&self.hostname) {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        
        if !resp.status().is_success() {
            return Err(anyhow!("API request failed with status: {}", resp.status()));
        }

        resp.json::<T>().await.map_err(|e| anyhow!("Failed to parse response: {}", e))
    }
}
