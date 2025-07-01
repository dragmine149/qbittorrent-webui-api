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
        let version = self
            ._get("/api/v2/app/version")
            .await?
            .send()
            .await?
            .text()
            .await?;

        Ok(version)
    }

    /// Get WebAPI version
    ///
    /// The response is a string with the WebAPI version, e.g. `2.11.4`
    pub async fn webapi_version(&self) -> Result<String, Error> {
        let version = self
            ._get("/api/v2/app/webapiVersion")
            .await?
            .send()
            .await?
            .text()
            .await?;

        Ok(version)
    }

    /// Get build info
    pub async fn build_info(&self) -> Result<BuildInfo, Error> {
        let build_info = self
            ._get("/api/v2/app/buildInfo")
            .await?
            .send()
            .await?
            .json::<BuildInfo>()
            .await?;

        Ok(build_info)
    }

    /// Shutdown Qbittorent application
    pub async fn shutdown(&self) -> Result<(), Error> {
        self._post("/api/v2/app/shutdown").await?.send().await?;

        Ok(())
    }

    /// Get application preferences
    ///
    /// Returns struct with several fields representing the application's settings.
    pub async fn preferences(&self) -> Result<Preferences, Error> {
        let build_info = self
            ._get("/api/v2/app/preferences")
            .await?
            .send()
            .await?
            .json::<Preferences>()
            .await?;

        Ok(build_info)
    }

    /// Set application preferences
    pub async fn set_preferences(&self, preferences: Preferences) -> Result<(), Error> {
        let mut form = multipart::Form::new();
        form = form.text("json", serde_json::to_string(&preferences)?);

        self._post("/api/v2/app/setPreferences")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }

    /// Get default save path
    pub async fn default_save_path(&self) -> Result<String, Error> {
        let preferences = self
            ._get("/api/v2/app/defaultSavePath")
            .await?
            .send()
            .await?
            .text()
            .await?;

        Ok(preferences)
    }

    /// Get cookies
    ///
    /// Retrieves cookies used for downloading .torrent files and RSS feeds.
    pub async fn cookies(&self) -> Result<Vec<Cookie>, Error> {
        let cookies = self
            ._get("/api/v2/app/cookies")
            .await?
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
        let mut form = multipart::Form::new();
        form = form.text("cookies", serde_json::to_string(&cookies)?);

        self._post("/api/v2/app/setCookies")
            .await?
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }
}
