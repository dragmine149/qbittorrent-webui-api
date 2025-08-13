use std::collections::HashMap;

use crate::{
    Error,
    models::{TorrentCreator, TorrentCreatorTask},
};

impl super::Api {
    pub async fn create_torrent(
        &self,
        params: &TorrentCreator,
    ) -> Result<TorrentCreatorTask, Error> {
        let mut form = HashMap::new();
        form.insert("sourcePath", params.source_path.clone());
        form.insert("format", format!("{}", params.format));
        form.insert("pieceSize", params.piece_size.0.to_string());
        form.insert("optimizeAlignment", params.optimize_alignment.to_string());
        form.insert(
            "paddedFileSizeLimit",
            params.padded_file_size_limit.to_string(),
        );
        form.insert("private", params.private.to_string());
        form.insert("start_seeding", params.start_seeding.to_string());
        if let Some(file_path) = &params.torrent_file_path {
            form.insert("torrentFilePath", file_path.clone());
        }
        if let Some(trackers) = &params.trackers {
            form.insert("trackers", trackers.join("|"));
        }
        if let Some(seeds) = &params.url_seeds {
            form.insert("trackers", seeds.join("|"));
        }
        if let Some(source) = &params.source {
            form.insert("trackers", source.clone());
        }
        if let Some(comment) = &params.comment {
            form.insert("trackers", comment.clone());
        }

        Ok(self
            ._post("torrentcreator/addTask")
            .await?
            .form(&form)
            // .body()
            .send()
            .await?
            .error_for_status()?
            .json::<TorrentCreatorTask>()
            .await?)
    }
}
