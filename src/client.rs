use std::{io::Error, str::FromStr};

use reqwest::{Client, Url};
use tokio::sync::RwLock;

pub struct QbitTorrentClient {
    http_client: Client,
    base_url: RwLock<Url>,
}

impl QbitTorrentClient {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let http_client = Client::new();

        let base_url = Url::from_str(url).unwrap();

        Ok(QbitTorrentClient {
            http_client: http_client,
            base_url: RwLock::new(base_url),
        })
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<(), Error> {
        let base_url = self.base_url.read().await;
        let url = base_url.join("/api/v2/auth/login").unwrap();

        let res = self
            .http_client
            .post(url)
            .body(format!("username={}&password={}", username, password))
            .header("refer", base_url.to_string())
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
        let base_url = self.base_url.read().await;
        let url = base_url.join("/api/v2/logout").unwrap();

        self.http_client.post(url).send().await.unwrap();

        Ok(())
    }
}
