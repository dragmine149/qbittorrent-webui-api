use reqwest::multipart;

use crate::error::Error;

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
}
