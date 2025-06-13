use crate::{error::Error, models::BuildInfo};

impl super::Client {
    pub async fn app_qbit_version(&self) -> Result<String, Error> {
        let url = self.build_url("/api/v2/app/version").await?;

        let version = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(version)
    }

    pub async fn app_webapi_version(&self) -> Result<String, Error> {
        let url = self.build_url("/api/v2/app/webapiVersion").await?;

        let version = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(version)
    }

    pub async fn app_build_info(&self) -> Result<BuildInfo, Error> {
        let url = self.build_url("/api/v2/app/buildInfo").await?;

        let build_info = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<BuildInfo>()
            .await?;

        Ok(build_info)
    }
}
