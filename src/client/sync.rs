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
        let mut query = vec![];
        if let Some(rid) = rid {
            query.push(("rid", rid));
        }

        let data = self
            ._get("/api/v2/sync/maindata")
            .await?
            .query(&query)
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
        let mut query = vec![];
        query.push(("hash", hash.to_string()));
        if let Some(rid) = rid {
            query.push(("rid", rid.to_string()));
        }

        let data = self
            ._get("/api/v2/sync/torrentPeers")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<PeersData>()
            .await?;

        Ok(data)
    }
}
