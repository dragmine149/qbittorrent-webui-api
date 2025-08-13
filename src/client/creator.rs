use crate::{
    Error,
    models::{TorrentCreator, TorrentCreatorTask},
};

impl super::Api {
    pub async fn create_torrent(
        &self,
        creator: &TorrentCreator,
    ) -> Result<TorrentCreatorTask, Error> {
        Ok(self
            ._post("torrentcreator/addTask")
            .await?
            .query(creator)
            .send()
            .await?
            .error_for_status()?
            .json::<TorrentCreatorTask>()
            .await?)
    }
}
