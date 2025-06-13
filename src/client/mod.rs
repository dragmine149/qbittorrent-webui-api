use core::str;
use std::str::FromStr;

use reqwest::{Client as ReqwestClient, Url};
use tokio::sync::RwLock;

mod authentication;
mod log;
mod sync;
mod torrent;
mod transfer;

use crate::error::Error;

pub use authentication::Creddentials;

pub struct Client {
    http_client: ReqwestClient,
    base_url: RwLock<Url>,
}

impl Client {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let http_client = ReqwestClient::builder().cookie_store(true).build()?;

        let base_url = Url::from_str(url)?;

        Ok(Self {
            http_client: http_client,
            base_url: RwLock::new(base_url),
        })
    }

    async fn build_url(&self, endpoint: &str) -> Result<Url, Error> {
        let base_url = self.base_url.read().await;
        let url = base_url.join(endpoint)?;

        Ok(url)
    }

    // ########################
    // Application
    // ########################
}
