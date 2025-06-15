use std::collections::HashMap;

use reqwest::multipart;

use crate::{
    error::Error,
    models::{
        FilePriority, PiecesState, TorrentContent, TorrentInfo, TorrentProperties, TorrentTracker,
        TorrentWebSeed,
    },
    parameters::{TorrentAddUrls, TorrentListParams},
};

impl super::Api {
    /// Get torrent list
    ///
    /// # Arguments
    ///
    /// * `parames` - Parameter object
    pub async fn torrents(&self, parames: TorrentListParams) -> Result<Vec<TorrentInfo>, Error> {
        let mut url = self._build_url("api/v2/torrents/info").await?;

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

    /// Get torrent generic properties
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the generic properties of.
    pub async fn torrent(&self, hash: &str) -> Result<TorrentProperties, Error> {
        let mut url = self._build_url("api/v2/torrents/properties").await?;
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

    /// Get torrent trackers
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the trackers of.
    pub async fn trackers(&self, hash: &str) -> Result<Vec<TorrentTracker>, Error> {
        let mut url = self._build_url("api/v2/torrents/trackers").await?;
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

    /// Get torrent web seeds
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the webseeds of.
    pub async fn webseeds(&self, hash: &str) -> Result<Vec<TorrentWebSeed>, Error> {
        let mut url = self._build_url("api/v2/torrents/webseeds").await?;
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

    /// Get torrent contents
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the files of.
    /// * `indexes` - The indexes of the files you want to retrieve. If `None`
    /// all files will be selected.
    pub async fn files(
        &self,
        hash: &str,
        indexes: Option<Vec<i64>>,
    ) -> Result<Vec<TorrentContent>, Error> {
        let mut url = self._build_url("api/v2/torrents/files").await?;

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

    /// Get torrent pieces' states
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the pice states of.
    pub async fn pieces_states(&self, hash: &str) -> Result<Vec<PiecesState>, Error> {
        let mut url = self._build_url("api/v2/torrents/pieceStates").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hash", &hash);
        drop(query);

        let pieces = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<PiecesState>>()
            .await?;

        Ok(pieces)
    }

    /// Get torrent pieces' hashes
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to get the pieces hashes of.
    pub async fn pieces_hashes(&self, hash: &str) -> Result<Vec<String>, Error> {
        let mut url = self._build_url("api/v2/torrents/pieceHashes").await?;

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

    /// Pause torrents
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to stop.
    pub async fn stop(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self._build_url("api/v2/torrents/stop").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    /// Resume torrents
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to start.
    pub async fn start(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self._build_url("api/v2/torrents/start").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    /// Delete torrents
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to delete.
    /// * `delete_files` - If set to `true`, the downloaded data will also be deleted,
    /// otherwise has no effect.
    pub async fn delete(&self, hashes: Vec<&str>, delete_files: bool) -> Result<(), Error> {
        let mut url = self._build_url("api/v2/torrents/delete").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        query.append_pair("deleteFiles", &delete_files.to_string());
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    /// Recheck torrents
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to recheck.
    pub async fn recheck(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self._build_url("api/v2/torrents/recheck").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    /// Reannounce torrents
    ///
    /// # Arguments
    ///
    /// * `hashes` - Hashes list of torrents to reannounce.
    pub async fn reannounce(&self, hashes: Vec<&str>) -> Result<(), Error> {
        let mut url = self._build_url("api/v2/torrents/reannounce").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hashes", &hashes.join("|"));
        drop(query);

        self.http_client.get(url).send().await?;

        Ok(())
    }

    /// Add new torrent
    ///
    /// # Arguments
    ///
    /// * `params` - Torrent parameters
    pub async fn add_torrent(&self, params: TorrentAddUrls) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/add").await?;

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

    /// Add trackers to torrent
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `urls` - Trackers urls to add.
    pub async fn add_trackers_to_torrent(&self, hash: &str, urls: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/addTrackers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("urls", urls.join("%0A"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Edit trackers
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent.
    /// * `orig_url` - The tracker URL you want to edit.
    /// * `new_url` - The new URL to replace the `origUrl`.
    pub async fn edit_tracker_for_torrent(
        &self,
        hash: &str,
        orig_url: &str,
        new_url: &str,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/editTracker").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("origUrl", orig_url.to_string());
        form = form.text("newUrl", new_url.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Remove trackers from torrent
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `urls` - Trackers urls to remove.
    pub async fn remove_trackers_from_torrent(
        &self,
        hash: &str,
        urls: Vec<&str>,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/removeTrackers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("urls", urls.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Remove trackers from torrent
    ///
    /// # Arguments
    ///
    /// * `hashes` - Torrent hash.
    /// * `peers` - The peer to add. Each peer is a colon-separated `host:port`.
    pub async fn add_peers(&self, hashes: Vec<&str>, peers: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/addPeers").await?;

        let mut form = multipart::Form::new();
        form = form.text("hashes", hashes.join("|"));
        form = form.text("peers", peers.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Increase torrent priority
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to increase the priority of.
    /// If `None` all torrents are selected.
    pub async fn increase_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/increasePrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Decrease torrent priority
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to decrease the priority of.
    /// If `None` all torrents are selected.
    pub async fn decrease_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/decreasePrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Maximal torrent priority
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to max the priority of.
    /// If `None` all torrents are selected.
    pub async fn max_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/topPrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Minimal torrent priority
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to min the priority of.
    /// If `None` all torrents are selected.
    pub async fn min_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/bottomPrio").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set file priority
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent.
    /// * `file_ids` - File ids.
    /// * `priority` - File priority to set.
    pub async fn set_file_priority(
        &self,
        hash: &str,
        file_ids: Vec<u64>,
        priority: FilePriority,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/filePrio").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text(
            "id",
            file_ids
                .iter()
                .map(|&num| num.to_string())
                .collect::<Vec<String>>()
                .join("|"),
        );
        form = form.text("priority", serde_json::to_string(&priority)?);

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get torrent download limit
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to get the download limit of.
    /// If `None` all torrents are selected.
    pub async fn download_limit(
        &self,
        hashes: Option<Vec<&str>>,
    ) -> Result<HashMap<String, u64>, Error> {
        let mut url = self._build_url("api/v2/torrents/downloadLimit").await?;

        let mut query = url.query_pairs_mut();
        if let Some(hashes) = hashes {
            query.append_pair("hashes", &hashes.join("|"));
        } else {
            query.append_pair("hashes", "all");
        }
        drop(query);

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<HashMap<String, u64>>()
            .await?;

        Ok(limites)
    }

    /// Set torrent download limit
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the download limit of.
    /// If `None` all torrents are selected.
    /// * `limit` - Download limit
    pub async fn set_download_limit(
        &self,
        hashes: Option<Vec<&str>>,
        limit: u64,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setDownloadLimit").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set torrent share limit
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the share limit of.
    /// If `None` all torrents are selected.
    /// * `ratio_limit` - The maximum seeding ratio for the torrent. `-2` means
    /// the global limit should be used, `-1` means no limit.
    /// * `seeding_time_limit` - The maximum seeding time (minutes) for the torrent.
    /// `-2` means the global limit should be used, `-1` means no limit.
    /// * `inactive_seeding_time_limit` - The maximum amount of time (minutes) the
    /// torrent is allowed to seed while being inactive. `-2` means the global limit
    /// should be used, `-1` means no limit.
    pub async fn set_share_limit(
        &self,
        hashes: Option<Vec<&str>>,
        ratio_limit: f64,
        seeding_time_limit: i64,
        inactive_seeding_time_limit: i64,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setShareLimits").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("ratioLimit", ratio_limit.to_string());
        form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
        form = form.text(
            "inactiveSeedingTimeLimit",
            inactive_seeding_time_limit.to_string(),
        );

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get torrent upload limit
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want the upload limit of.
    /// If `None` all torrents are selected.
    pub async fn upload_limit(
        &self,
        hashes: Option<Vec<&str>>,
    ) -> Result<HashMap<String, i64>, Error> {
        let mut url = self._build_url("api/v2/torrents/uploadLimit").await?;

        let mut query = url.query_pairs_mut();
        if let Some(hashes) = hashes {
            query.append_pair("hashes", &hashes.join("|"));
        } else {
            query.append_pair("hashes", "all");
        }
        drop(query);

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<HashMap<String, i64>>()
            .await?;

        Ok(limites)
    }

    /// Set torrent upload limit
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the upload limit of.
    /// If `None` all torrents are selected.
    /// * `limit` - Upload limit
    pub async fn set_upload_limit(
        &self,
        hashes: Option<Vec<&str>>,
        limit: u64,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setUploadLimit").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set torrent location
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the location of.
    /// If `None` all torrents are selected.
    /// * `location` - Location to download the torrent to.
    pub async fn set_location(
        &self,
        hashes: Option<Vec<&str>>,
        location: &str,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setLocation").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("location", location.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set torrent name
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent you want to set the name of.
    /// * `name` - Location to download the torrent to.
    pub async fn set_name(&self, hash: &str, name: &str) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setLocation").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("name", name.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set torrent category
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the category of.
    /// If `None` all torrents are selected.
    /// * `category` - Name of the category you want to set.
    pub async fn set_category(
        &self,
        hashes: Option<Vec<&str>>,
        category: &str,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setCategory").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("category", category.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get all categories
    pub async fn categories(&self) -> Result<Vec<String>, Error> {
        let url = self._build_url("api/v2/torrents/categories").await?;

        let categories = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(categories)
    }

    /// Add new category
    ///
    /// # Arguments
    ///
    /// * `category` - Name for the category to create.
    /// * `save_path` - Path to download torrents for the category.
    pub async fn create_category(&self, category: &str, save_path: &str) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/createCategory").await?;

        let mut form = multipart::Form::new();
        form = form.text("category", category.to_string());
        form = form.text("savePath", save_path.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Edit category
    ///
    /// # Arguments
    ///
    /// * `category` - Name for the category to edit.
    /// * `save_path` - Path to download torrents for the category.
    pub async fn edit_category(&self, category: &str, save_path: &str) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/editCategory").await?;

        let mut form = multipart::Form::new();
        form = form.text("category", category.to_string());
        form = form.text("savePath", save_path.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Remove categories
    ///
    /// # Arguments
    ///
    /// * `categories` - List of category names to remove.
    pub async fn remove_categories(&self, categories: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/removeCategories").await?;

        let mut form = multipart::Form::new();
        form = form.text("categories", categories.join("\n"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Add torrent tags
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set the tags of.
    /// If `None` all torrents are selected.
    /// * `tags` - List of names for the tags you want to set.
    pub async fn add_tags(&self, hashes: Option<Vec<&str>>, tags: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/addTags").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("tags", tags.join(","));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Remove torrent tags
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to remove the tags of.
    /// If `None` all torrents are selected.
    /// * `tags` - List of names for the tags you want to remove.
    pub async fn remove_tags(
        &self,
        hashes: Option<Vec<&str>>,
        tags: Vec<&str>,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/removeTags").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("tags", tags.join(","));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get all tags
    pub async fn tags(&self) -> Result<Vec<String>, Error> {
        let url = self._build_url("api/v2/torrents/tags").await?;

        let tags = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(tags)
    }

    /// Create tags
    ///
    /// # Arguments
    ///
    /// * `tags` - List of tags to create.
    pub async fn create_tags(&self, tags: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/createTags").await?;

        let mut form = multipart::Form::new();
        form = form.text("tags", tags.join(","));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Delete tags
    ///
    /// # Arguments
    ///
    /// * `tags` - List of tags to delete.
    pub async fn torrent_delete_tags(&self, tags: Vec<&str>) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/deleteTags").await?;

        let mut form = multipart::Form::new();
        form = form.text("tags", tags.join(","));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set automatic torrent management
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set automatic torrent management of.
    /// If `None` all torrents are selected.
    /// * `enable`
    pub async fn set_automatic_torrent_managment(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setAutoManagement").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("enable", enable.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Toggle sequential download
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to toggle sequential download for.
    /// If `None` all torrents are selected.
    pub async fn sequential_download(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self
            ._build_url("api/v2/torrents/toggleSequentialDownload")
            .await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Toggle first/last piece priority
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to toggle first/last piece priority for.
    /// If `None` all torrents are selected.
    pub async fn toggle_first_last_priority(&self, hashes: Option<Vec<&str>>) -> Result<(), Error> {
        let url = self
            ._build_url("api/v2/torrents/toggleFirstLastPiecePrio")
            .await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set force start
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set force start of.
    /// If `None` all torrents are selected.
    /// * `enable`
    pub async fn set_force_start(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setForceStart").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("value", enable.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Set super seeding
    ///
    /// # Arguments
    ///
    /// * `hashes` - The hashes of the torrents you want to set super seeding of.
    /// If `None` all torrents are selected.
    /// * `enable`
    pub async fn set_super_seeding(
        &self,
        hashes: Option<Vec<&str>>,
        enable: bool,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/setSuperSeeding").await?;

        let mut form = multipart::Form::new();
        if let Some(hashes) = hashes {
            form = form.text("hashes", hashes.join("|"));
        } else {
            form = form.text("hashes", "all".to_string());
        }
        form = form.text("value", enable.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Rename file
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent
    /// * `oldPath` - The old path of the torrent
    /// * `newPath` - The new path to use for the file
    pub async fn rename_file(
        &self,
        hash: &str,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/renameFile").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("oldPath", old_path.to_string());
        form = form.text("newPath", new_path.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Rename folder
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash of the torrent
    /// * `oldPath` - The old path of the torrent
    /// * `newPath` - The new path to use for the file
    pub async fn torrent_rename_folder(
        &self,
        hash: &str,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), Error> {
        let url = self._build_url("api/v2/torrents/renameFolder").await?;

        let mut form = multipart::Form::new();
        form = form.text("hash", hash.to_string());
        form = form.text("oldPath", old_path.to_string());
        form = form.text("newPath", new_path.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }
}
