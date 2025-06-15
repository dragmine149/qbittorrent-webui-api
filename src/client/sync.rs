use crate::{
    error::Error,
    models::{MainData, PeersData},
};

impl super::Api {
    /// Get main data
    ///
    /// If the given `rid` is different from the one of last server reply,
    /// `full_update` will be `true`
    ///
    /// # Arguments
    ///
    /// * `rid` - Response ID. If not provided, `rid=0` will be assumed.
    pub async fn main_data(&self, rid: Option<i64>) -> Result<MainData, Error> {
        let mut url = self._build_url("/api/v2/sync/maindata").await?;

        let mut query = url.query_pairs_mut();
        if let Some(rid) = rid {
            query.append_pair("rid", &rid.to_string());
        }
        drop(query);

        let data = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<MainData>()
            .await?;

        Ok(data)
    }

    /// Get torrent peers data
    ///
    /// Fetches main data changes since the last request. If the given `rid` is different from the one of last server reply,
    /// `full_update` will be `true`
    ///
    /// # Arguments
    ///
    /// * `hash` - Torrent hash.
    /// * `rid` - Response ID. If not provided, `rid=0` will be assumed.
    pub async fn peers_data(&self, hash: &str, rid: Option<i64>) -> Result<PeersData, Error> {
        let mut url = self._build_url("/api/v2/sync/torrentPeers").await?;

        let mut query = url.query_pairs_mut();
        query.append_pair("hash", &hash.to_string());
        if let Some(rid) = rid {
            query.append_pair("rid", &rid.to_string());
        }
        drop(query);

        let data = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<PeersData>()
            .await?;

        Ok(data)
    }
}
