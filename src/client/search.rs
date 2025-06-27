use reqwest::multipart;

use crate::{
    error::Error,
    models::{Search, SearchResult},
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
        let url = self._build_url("/api/v2/search/start").await?;

        let mut form = multipart::Form::new();
        form = form.text("pattern", pattern.to_string());
        form = form.text("plugins", plugins.to_string());
        form = form.text("category", category.to_string());

        let response = self.http_client.post(url).multipart(form).send().await?;
        let json: serde_json::Value = response.json().await?;
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
        let url = self._build_url("/api/v2/search/stop").await?;

        let mut form = multipart::Form::new();
        form = form.text("id", id.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get search status
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the search job. If `None`, all search jobs are returned
    pub async fn search_status(&self, id: Option<u64>) -> Result<Vec<Search>, Error> {
        let mut url = self._build_url("api/v2/search/status").await?;

        let mut query = url.query_pairs_mut();
        if let Some(id) = id {
            query.append_pair("id", &id.to_string());
        }
        drop(query);

        let searches = self
            .http_client
            .get(url)
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
    /// # Parameters
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
        let mut url = self._build_url("api/v2/search/results").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("id", &id.to_string());
        query.append_pair("limit", &limit.to_string());
        if let Some(offset) = offset {
            query.append_pair("offset", &offset.to_string());
        }
        drop(query);

        let searches = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<SearchResult>()
            .await?;

        Ok(searches)
    }
}
