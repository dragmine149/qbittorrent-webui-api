use core::str;
use std::str::FromStr;

use reqwest::{Client as ReqwestClient, Url, multipart};
use tokio::sync::RwLock;

use crate::{
    error::Error,
    models::{
        FilePriority, LogPeers, TorrentContent, TorrentInfo, TorrentProperties, TorrentTracker,
        TorrentWebSeed,
    },
    parames::{
        TorrentAddPeers, TorrentAddUrls, TorrentListParams, TorrentTrackersEdit,
        TorrentTrackersList,
    },
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

    // ########################
    // Authentication
    // ########################

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
    // Application
    // ########################

    // ########################
    // Log
    // ########################

    pub async fn log_peer(&self, last_known_id: Option<usize>) -> Result<Vec<LogPeers>, Error> {
        let mut url = self.build_url("api/v2/log/peers").await?;
        if let Some(id) = last_known_id {
            url.set_query(Some(&format!("last_known_id={}", id)));
        }

        let log = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<LogPeers>>()
            .await?;

        Ok(log)
    }

    // ########################
    // Sync
    // ########################

    pub async fn torrent_peers_data(&self, _hash: &str, _rid: usize) -> Result<Vec<()>, Error> {
        todo!("Not added. Documentaion missing!")
    }

    // ########################
    // Transfer info
    // ########################

    pub async fn transfer_peers_ban(&self, peers: Vec<String>) -> Result<(), Error> {
        let url = self.build_url("api/v2/transfer/banPeers").await?;

        let mut form = multipart::Form::new();
        form = form.text("peers", peers.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    // ########################
    // Torrent management
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

    pub async fn torrent_pieces_states(&self, hash: &str) -> Result<Vec<u8>, Error> {
        let mut url = self.build_url("api/v2/torrents/pieceStates").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hash", &hash);
        drop(query);

        let pieces = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<u8>>()
            .await?;

        Ok(pieces)
    }

    pub async fn torrent_pieces_hashes(&self, hash: &str) -> Result<Vec<String>, Error> {
        let mut url = self.build_url("api/v2/torrents/pieceHashes").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hash", &hash);
        drop(query);

        let pieces = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(pieces)
    }

    pub async fn torrent_stop(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self.build_url("api/v2/torrents/stop").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    pub async fn torrent_start(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self.build_url("api/v2/torrents/start").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    pub async fn torrent_delete(&self, hashes: Vec<&str>, delete_files: bool) -> Result<(), Error> {
        let mut url = self.build_url("api/v2/torrents/delete").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        query.append_pair("deleteFiles", &delete_files.to_string());
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    pub async fn torrent_recheck(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self.build_url("api/v2/torrents/recheck").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    pub async fn torrent_reannounce(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self.build_url("api/v2/torrents/reannounce").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    pub async fn torrent_add(&self, params: TorrentAddUrls) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/add").await?;

        let mut form = multipart::Form::new();
        form = form.text("urls", params.urls.join("\n"));
        form = form.text("skip_checking", params.skip_checking.to_string());
        form = form.text("paused", params.paused.to_string());
        form = form.text("autoTMM", params.auto_tmm.to_string());
        form = form.text("sequentialDownload", params.sequential_download.to_string());
        form = form.text(
            "firstLastPiecePrio",
            params.first_last_piece_prio.to_string(),
        );
        if let Some(savepath) = params.savepath {
            form = form.text("savepath", savepath);
        }
        if let Some(category) = params.category {
            form = form.text("category", category);
        }
        if let Some(tags) = params.tags {
            form = form.text("tags", tags.join(","));
        }
        if let Some(root_folder) = params.root_folder {
            form = form.text("root_folder", root_folder);
        }
        if let Some(rename) = params.rename {
            form = form.text("rename", rename);
        }
        if let Some(up_limit) = params.up_limit {
            form = form.text("upLimit", up_limit.to_string());
        }
        if let Some(dl_limit) = params.dl_limit {
            form = form.text("dlLimit", dl_limit.to_string());
        }
        if let Some(ratio_limit) = params.ratio_limit {
            form = form.text("ratioLimit", ratio_limit.to_string());
        }
        if let Some(seeding_time_limit) = params.seeding_time_limit {
            form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_trackers_add(&self, params: TorrentTrackersList) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/addTrackers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", params.hash);
        form = form.text("urls", params.urls.join("%0A"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_trackers_edit(&self, params: TorrentTrackersEdit) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/editTracker").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", params.hash);
        form = form.text("origUrl", params.orig_url);
        form = form.text("newUrl", params.new_url);

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_trackers_delete(&self, params: TorrentTrackersList) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/removeTrackers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", params.hash);
        form = form.text("urls", params.urls.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_peers_add(&self, params: TorrentAddPeers) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/addPeers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hashes", params.hashes.join("|"));
        form = form.text("peers", params.peers.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_increase_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/increasePrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_decrease_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/decreasePrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_max_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/topPrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_min_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/bottomPrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn torrent_file_priority(
        &self,
        hash: &str,
        id: usize,
        priority: FilePriority,
    ) -> Result<(), Error> {
        let url = self.build_url("api/v2/torrents/filePrio").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("id", id.to_string());
        form = form.text("priority", serde_json::to_string(&priority)?);

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    // ########################
    // RSS
    // ########################

    // ########################
    // Search
    // ########################
}
