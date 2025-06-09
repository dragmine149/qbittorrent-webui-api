use core::str;
use std::{io::Error, str::FromStr};

use reqwest::{Client as ReqwestClient, Url};
use tokio::sync::RwLock;

pub struct Creddentials {
    username: String,
    password: String,
}

pub struct Client {
    http_client: ReqwestClient,
    base_url: RwLock<Url>,
}

impl Client {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let http_client = ReqwestClient::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let base_url = Url::from_str(url).unwrap();

        Ok(Self { 
            http_client: http_client,
            base_url: RwLock::new(base_url),
        })
    }

    async fn build_url(&self, endpoint: &str) -> Result<Url, Error> {
        let base_url = self.base_url.read().await;
        let url = base_url.join(endpoint).unwrap(); 
        
        Ok(url)
    }

    pub async fn login(&self, cred: Creddentials) -> Result<(), Error> {
        let url = self.build_url("/api/v2/auth/login").await.unwrap();

        let res = self
            .http_client
            .post(url)
            .body(format!("username={}&password={}", cred.username, cred.password))
            .header("refer", self.base_url.read().await.to_string())
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Login failed",
            ))
        }
    }

    pub async fn logout(&self) -> Result<(), Error> {
        let url = self.build_url("/api/v2/logout").await.unwrap();

        self.http_client.post(url).send().await.unwrap();

        Ok(())
    }
}
