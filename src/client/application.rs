use reqwest::multipart;

use crate::{
    error::Error,
    models::{BuildInfo, Cookie, Preferences},
};

impl super::Api {
    /// Get Qbittorrent application version
    ///
    /// The response is a string withe the application version, e.g. `v5.1.0`
    pub async fn version(&self) -> Result<String, Error> {
        let url = self._build_url("/api/v2/app/version").await?;

        let version = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(version)
    }

    /// Get WebAPI version
    ///
    /// The response is a string with the WebAPI version, e.g. `2.11.4`
    pub async fn webapi_version(&self) -> Result<String, Error> {
        let url = self._build_url("/api/v2/app/webapiVersion").await?;

        let version = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(version)
    }

    /// Get build info
    pub async fn build_info(&self) -> Result<BuildInfo, Error> {
        let url = self._build_url("/api/v2/app/buildInfo").await?;

        let build_info = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<BuildInfo>()
            .await?;

        Ok(build_info)
    }

    /// Shutdown Qbittorent application
    pub async fn shutdown(&self) -> Result<(), Error> {
        let url = self._build_url("/api/v2/app/shutdown").await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }

    /// Get application preferences
    ///
    /// Returns struct with several fields representing the application's settings.
    pub async fn preferences(&self) -> Result<Preferences, Error> {
        let url = self._build_url("/api/v2/app/preferences").await?;

        let preferences = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Preferences>()
            .await?;

        Ok(preferences)
    }

    /// Set application preferences
    pub async fn set_preferences(&self, preferences: Preferences) -> Result<(), Error> {
        let url = self._build_url("/api/v2/app/setPreferences").await?;

        let mut form = multipart::Form::new();
        form = form.text("json", serde_json::to_string(&preferences)?);

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get default save path
    pub async fn default_save_path(&self) -> Result<String, Error> {
        let url = self._build_url("/api/v2/app/defaultSavePath").await?;

        let preferences = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(preferences)
    }

    /// Get cookies
    ///
    /// Retrieves cookies used for downloading .torrent files and RSS feeds.
    pub async fn cookies(&self) -> Result<Vec<Cookie>, Error> {
        let url = self._build_url("/api/v2/app/cookies").await?;

        let cookies = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<Cookie>>()
            .await?;

        Ok(cookies)
    }

    /// Set cookies
    ///
    /// Sets the cookies used for downloading .torrent files and RSS feeds.
    ///
    /// This will overwrite all the cookies.
    ///
    /// # Arguments
    ///
    /// * `cookies` - A list of cookies to be set.
    pub async fn set_cookies(&self, cookies: Vec<Cookie>) -> Result<(), Error> {
        let url = self._build_url("/api/v2/app/setCookies").await?;

        let mut form = multipart::Form::new();
        form = form.text("cookies", serde_json::to_string(&cookies)?);

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }
}
