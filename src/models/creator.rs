use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TorrentFormat {
    V1,
    V2,
    Hybrid,
}

impl Default for TorrentFormat {
    fn default() -> Self {
        Self::Hybrid
    }
}

#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Builder,
)]
pub struct TorrentCreator {
    #[builder(setter(into))]
    pub source_path: PathBuf,
    #[builder(setter(into), default)]
    pub format: TorrentFormat,
    /// How big a piece of the file is. (in Bytes). 0 = auto.
    #[builder(default)]
    pub piece_size: u64,
    pub optimize_alignment: bool,
    pub padded_file_size_limit: TorrentPieceSize,
    pub private: bool,
    pub start_seeding: bool,
    pub torrent_file_path: String,
    pub trackers: Vec<String>,
    pub url_seeds: Vec<String>,
    pub source: String,
    pub comment: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentCreatorTask {
    pub task_id: String,
}
