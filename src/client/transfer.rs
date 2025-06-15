use reqwest::multipart;

use crate::{error::Error, models::TransferInfo};

impl super::Api {
    /// Get global transfer info
    ///
    /// This method returns info you usually see in qBt status bar.
    pub async fn global_transfer_info(&self) -> Result<TransferInfo, Error> {
        let url = self._build_url("api/v2/transfer/info").await?;

        let info = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<TransferInfo>()
            .await?;

        Ok(info)
    }

    /// Get alternative speed limits state
    ///
    /// The response is 1 if alternative speed limits are enabled, 0 otherwise.
    pub async fn alternative_speed_limit(&self) -> Result<u8, Error> {
        let url = self._build_url("api/v2/transfer/speedLimitsMode").await?;

        let is_active = self.http_client.get(url).send().await?.json::<u8>().await?;

        Ok(is_active)
    }

    /// Toggle alternative speed limits
    pub async fn toggle_alternative_speed_limit(&self) -> Result<(), Error> {
        let url = self
            ._build_url("api/v2/transfer/toggleSpeedLimitsMode")
            .await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }

    /// Get global download limit
    pub async fn global_download_limit(&self) -> Result<u64, Error> {
        let url = self._build_url("api/v2/transfer/downloadLimit").await?;

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<u64>()
            .await?;

        Ok(limites)
    }

    /// Set global download limit
    ///
    /// # Arguments
    ///
    /// * `limit` - The global download speed limit to set in bytes/second. `0` if no limit.
    pub async fn set_global_download_limit(&self, limit: u64) -> Result<(), Error> {
        let url = self._build_url("api/v2/transfer/setDownloadLimit").await?;

        let mut form = multipart::Form::new();
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Get global upload limit
    pub async fn global_upload_limit(&self) -> Result<u64, Error> {
        let url = self._build_url("api/v2/transfer/uploadLimit").await?;

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<u64>()
            .await?;

        Ok(limites)
    }

    /// Set global upload limit
    ///
    /// # Arguments
    ///
    /// * `limit` - The global upload speed limit to set in bytes/second. `0` if no limit.
    pub async fn set_global_upload_limit(&self, limit: u64) -> Result<(), Error> {
        let url = self._build_url("api/v2/transfer/setUploadLimit").await?;

        let mut form = multipart::Form::new();
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    /// Ban peers
    ///
    /// # Arguments
    ///
    /// * `peers` - The peer to ban, or multiple peers. Each peer is a colon-separated `host:port`
    pub async fn peers_ban(&self, peers: Vec<String>) -> Result<(), Error> {
        let url = self._build_url("api/v2/transfer/banPeers").await?;

        let mut form = multipart::Form::new();
        form = form.text("peers", peers.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }
}
