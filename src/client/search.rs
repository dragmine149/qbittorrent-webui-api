use reqwest::multipart;

use crate::{
    error::Error,
    models::{Search, SearchPlugin, SearchResult},
};

impl super::Api {
    /// Start search
    ///
    /// # Arguments
    /// * `pattern` - Pattern to search for (e.g. "Ubuntu 18.04")
    /// * `plugins` - Plugins to use for searching (e.g. "legittorrents"). Supports
    /// multiple plugins separated by `|`. Also supports `all` and `enabled`
    /// * `category` - Categories to limit your search to (e.g. "legittorrents").
    /// Available categories depend on the specified plugins. Also supports `all`
    pub async fn search_start(
        &self,
        pattern: &str,
        plugins: &str,
        category: &str,
    ) -> Result<u64, Error> {
        let mut form = multipart::Form::new();
        form = form.text("pattern", pattern.to_string());
        form = form.text("plugins", plugins.to_string());
        form = form.text("category", category.to_string());

        let json: serde_json::Value = self
            ._post("search/start")
            .await?
            .multipart(form)
            .send()
            .await?
            .json()
            .await?;
        let id = json["id"].as_u64().ok_or_else(|| {
            Error::InvalidResponse("Missing or invalid 'id' in response".to_string())
        })?;

        Ok(id)
    }

    /// Stop search
    ///
    /// # Arguments
    /// * `id` - ID of the search job
    pub async fn search_stop(&self, id: u64) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("id", id.to_string());

        self._post("search/stop")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Get search status
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the search job. If `None`, all search jobs are returned
    pub async fn search_status(&self, id: Option<u64>) -> Result<Vec<Search>, Error> {
        let mut query = vec![];
        if let Some(id) = id {
            query.push(("id", id));
        }

        let searches = self
            ._get("search/status")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<Vec<Search>>()
            .await?;

        Ok(searches)
    }

    /// Get search results
    ///
    /// This function retrieves search results for a given search job.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the search job.
    /// * `limit` - The maximum number of results to return. A value of `0` indicates no limit.
    /// * `offset` - The starting point for results. If negative, counts backwards (e.g., `-2` retrieves the 2 most recent results).
    pub async fn search_results(
        &self,
        id: u64,
        limit: u64,
        offset: Option<i64>,
    ) -> Result<SearchResult, Error> {
        let mut query = vec![];
        query.push(("id", id.to_string()));
        query.push(("limit", limit.to_string()));
        if let Some(offset) = offset {
            query.push(("offset", offset.to_string()));
        }

        let searches = self
            ._get("search/results")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<SearchResult>()
            .await?;

        Ok(searches)
    }

    /// Delete search
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the search job.
    pub async fn search_delete(&self, id: u64) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("id", id.to_string());

        self._post("search/delete")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Get search plugins
    pub async fn search_plugins(&self) -> Result<Vec<SearchPlugin>, Error> {
        let plugins = self
            ._get("search/plugins")
            .await?
            .send()
            .await?
            .json::<Vec<SearchPlugin>>()
            .await?;

        Ok(plugins)
    }

    /// Install search plugin
    ///
    /// # Arguments
    /// * `sources` - List of Url and file path of the plugin to install.
    pub async fn search_install_plugin(&self, sources: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("sources", sources.join("|"));

        self._post("search/installPlugin")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Uninstall search plugin
    ///
    /// # Arguments
    /// * `names` - List of names for torrents to uninstall.
    pub async fn search_uninstall_plugin(&self, names: Vec<&str>) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("names", names.join("|"));

        self._post("search/uninstallPlugin")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Enable search plugin
    ///
    /// # Arguments
    /// * `names` - List of names for torrents to enable.
    pub async fn search_enable_plugin(&self, names: Vec<&str>, enable: bool) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("names", names.join("|"));
        form = form.text("enable", enable.to_string());

        self._post("search/enablePlugin")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Update search plugins
    pub async fn search_update_plugin(&self) -> Result<(), Error> {
        self._post("search/updatePlugins").await?.send().await?;

        Ok(())
    }
}
