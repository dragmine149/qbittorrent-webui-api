use crate::{
    error::Error,
    models::{MainData, PeersData},
};

impl super::Client {
    pub async fn sync_main_data(&self, rid: Option<usize>) -> Result<MainData, Error> {
        let mut url = self.build_url("/api/v2/sync/maindata").await?;

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

    // The documentation was incomplete, so I constructed this based on the responses from the API.
    // Fields might be missing or broken
    pub async fn sync_peers_data(
        &self,
        hash: &str,
        rid: Option<usize>,
    ) -> Result<PeersData, Error> {
        let mut url = self.build_url("/api/v2/sync/torrentPeers").await?;

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
