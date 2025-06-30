use core::str;
use std::{str::FromStr, sync::Arc};

use cookie_store::Cookie;

use reqwest::{Client as ReqwestClient, Url};
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};

use crate::error::Error;

mod application;
mod authentication;
mod log;
mod rss;
mod search;
mod sync;
mod torrent;
mod transfer;

/// Represents a client for interacting with a remote API, handling HTTP requests.
pub struct Api {
    http_client: ReqwestClient,
    base_url: tokio::sync::RwLock<Url>,
    cookie_store: Arc<CookieStoreMutex>,
}

impl Api {
    /// Creates a new `API` instance.
    pub fn new(url: &str) -> Result<Self, Error> {
        let cookie_store = CookieStore::new(None);

        Self::_new_withe_cookie(url, cookie_store)
    }

    fn _new_withe_cookie(url: &str, cookie_store: CookieStore) -> Result<Self, Error> {
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        let http_client = ReqwestClient::builder()
            .cookie_provider(cookie_store.clone())
            .build()?;

        let base_url = Url::from_str(url)?;

        Ok(Self {
            http_client: http_client,
            base_url: tokio::sync::RwLock::new(base_url),
            cookie_store: cookie_store,
        })
    }

    /// Helper for constructing API URLs
    async fn _build_url(&self, endpoint: &str) -> Result<Url, Error> {
        let base_url = self.base_url.read().await;
        let url = base_url.join(endpoint)?;

        Ok(url)
    }

    pub async fn get_sid_cookie(&self) -> Option<String> {
        let base_url = self.base_url.read().await.clone();
        let cookie_store = self.cookie_store.lock().unwrap();
        let cookie = cookie_store.get(base_url.domain().unwrap(), base_url.path(), "SID");

        if let Some(x) = cookie {
            Some(x.value().to_string())
        } else {
            None
        }
    }

    pub async fn set_sid_cookie(&self, value: impl Into<String>) -> Result<(), Error> {
        let base_url = self.base_url.read().await.clone();
        let new_cookie = Cookie::parse(
            format!("SID={}; HttpOnly; SameSite=Strict; path=/", value.into()),
            &base_url,
        )?;

        let mut cookie_store = self.cookie_store.lock().unwrap();
        cookie_store.insert(new_cookie, &base_url)?;

        Ok(())
    }
}
