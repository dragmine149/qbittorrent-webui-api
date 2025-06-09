use core::str;
use std::str::FromStr;

use reqwest::{Client as ReqwestClient, Url};
use tokio::sync::RwLock;

use crate::{
    error::Error,
    models::{TorrentContent, TorrentInfo, TorrentProperties, TorrentTracker, TorrentWebSeed},
    parames::TorrentListParams,
};

#[derive(Debug)]
pub struct Creddentials {
    username: String,
    password: String,
}

impl Creddentials {
    pub fn new<T: ToString>(username: T, password: T) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub fn quary_string(&self) -> String {
        return format!("username={}&password={}", self.username, self.password);
    }
}

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

    pub async fn login(&self, cred: Creddentials) -> Result<(), Error> {
        let url = self.build_url("/api/v2/auth/login").await?;
        let res = self
            .http_client
            .post(url)
            .body(cred.quary_string())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("refer", self.base_url.read().await.to_string())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(Error::AuthFailed)
        }
    }

    pub async fn logout(&self) -> Result<(), Error> {
        let url = self.build_url("/api/v2/logout").await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }

    // ########################
    // Torrents
    // ########################

    pub async fn torrent_list(
        &self,
        parames: TorrentListParams,
    ) -> Result<Vec<TorrentInfo>, Error> {
        let mut url = self.build_url("api/v2/torrents/info").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("reverse", &parames.reverse.to_string());
        if let Some(filter) = parames.filter {
            query.append_pair("filter", &filter.to_string());
        }
        if let Some(category) = parames.category {
            query.append_pair("category", &category);
        }
        if let Some(tag) = parames.tag {
            query.append_pair("tag", &tag);
        }
        if let Some(sort) = parames.sort {
            query.append_pair("sort", &sort.to_string());
        }
        if let Some(limit) = parames.limit {
            query.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = parames.offset {
            query.append_pair("offset", &offset.to_string());
        }
        if let Some(hashes) = parames.hashes {
            query.append_pair("hashes", &hashes.join("|"));
        }
        drop(query);

        let torrents = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<TorrentInfo>>()
            .await?;

        Ok(torrents)
    }

    pub async fn torrent_properties(&self, hash: &str) -> Result<TorrentProperties, Error> {
        let mut url = self.build_url("api/v2/torrents/properties").await?;
        url.set_query(Some(&format!("hash={}", hash)));

        let torrent = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<TorrentProperties>()
            .await?;

        Ok(torrent)
    }

    pub async fn torrent_trackers(&self, hash: &str) -> Result<Vec<TorrentTracker>, Error> {
        let mut url = self.build_url("api/v2/torrents/trackers").await?;
        url.set_query(Some(&format!("hash={}", hash)));

        let trackers = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<TorrentTracker>>()
            .await?;

        Ok(trackers)
    }

    pub async fn torrent_webseeds(&self, hash: &str) -> Result<Vec<TorrentWebSeed>, Error> {
        let mut url = self.build_url("api/v2/torrents/webseeds").await?;
        url.set_query(Some(&format!("hash={}", hash)));

        let webseeds = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<TorrentWebSeed>>()
            .await?;

        Ok(webseeds)
    }

    pub async fn torrent_contents(
        &self,
        hash: &str,
        indexes: Option<Vec<usize>>,
    ) -> Result<Vec<TorrentContent>, Error> {
        let mut url = self.build_url("api/v2/torrents/files").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hash", &hash);
        if let Some(indexes) = indexes {
            query.append_pair(
                "filter",
                &indexes
                    .iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("|"),
            );
        }
        drop(query);

        let webseeds = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<TorrentContent>>()
            .await?;

        Ok(webseeds)
    }
}
