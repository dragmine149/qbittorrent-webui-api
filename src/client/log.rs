use crate::{error::Error, models::LogPeers};

impl super::Client {
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
}
