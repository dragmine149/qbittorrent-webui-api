use reqwest::multipart;

use crate::error::Error;

impl super::Client {
    pub async fn transfer_get_alternative_speed_limit(&self) -> Result<u8, Error> {
        let url = self.build_url("api/v2/transfer/speedLimitsMode").await?;

        let is_active = self.http_client.get(url).send().await?.json::<u8>().await?;

        Ok(is_active)
    }

    pub async fn transfer_toggle_alternative_speed_limit(&self) -> Result<(), Error> {
        let url = self
            .build_url("api/v2/transfer/toggleSpeedLimitsMode")
            .await?;

        self.http_client.post(url).send().await?;

        Ok(())
    }

    pub async fn transfer_get_global_download_limit(&self) -> Result<usize, Error> {
        let url = self.build_url("api/v2/transfer/downloadLimit").await?;

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<usize>()
            .await?;

        Ok(limites)
    }

    pub async fn transfer_set_global_download_limit(&self, limit: usize) -> Result<(), Error> {
        let url = self.build_url("api/v2/transfer/setDownloadLimit").await?;

        let mut form = multipart::Form::new();
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn transfer_get_global_upload_limit(&self) -> Result<usize, Error> {
        let url = self.build_url("api/v2/transfer/uploadLimit").await?;

        let limites = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<usize>()
            .await?;

        Ok(limites)
    }

    pub async fn transfer_set_global_upload_limit(&self, limit: usize) -> Result<(), Error> {
        let url = self.build_url("api/v2/transfer/setUploadLimit").await?;

        let mut form = multipart::Form::new();
        form = form.text("limit", limit.to_string());

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn transfer_peers_ban(&self, peers: Vec<String>) -> Result<(), Error> {
        let url = self.build_url("api/v2/transfer/banPeers").await?;

        let mut form = multipart::Form::new();
        form = form.text("peers", peers.join("|"));

        self.http_client.post(url).multipart(form).send().await?;

        Ok(())
    }
}
