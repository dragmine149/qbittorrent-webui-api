use std::path::PathBuf;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// The format of the torrent.
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

/// Everything required to create a new torrent.
#[derive(
    Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Builder,
)]
pub struct TorrentCreator {
    #[builder(setter(into))]
    /// Source file (or directory) of current torrent.
    pub source_path: PathBuf,
    #[builder(setter(into), default)]
    pub format: TorrentFormat,
    /// How big a piece of the file is. (in Bytes). 0 = auto.
    #[builder(setter(into), default)]
    pub piece_size: TorrentPieceSize,
    #[builder(default)]
    pub optimize_alignment: bool,
    #[builder(default)]
    pub padded_file_size_limit: u64,
    #[builder(default)]
    pub private: bool,
    #[builder(default)]
    /// To start seeding the torrent as soon as the file is created.
    pub start_seeding: bool,
    #[builder(setter(into, strip_option), default)]
    /// The filepath of the `.torrent` file to save to.
    pub torrent_file_path: Option<PathBuf>,
    #[builder(setter(into, strip_option), default)]
    pub trackers: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    pub url_seeds: Option<Vec<String>>,
    #[builder(setter(into, strip_option), default)]
    pub source: Option<String>,
    /// A comment to attach to the torrent.
    #[builder(setter(into, strip_option), default)]
    pub comment: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentCreatorTask {
    /// The task id related to the torrent just created
    pub task_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TorrentPieceSize(u64);

impl Default for TorrentPieceSize {
    fn default() -> Self {
        Self::auto()
    }
}

impl TorrentPieceSize {
    /// Leave it up to qbittorrent to decided.
    pub fn auto() -> Self {
        Self(0)
    }
    /// Pieces in 16 KibiBytes (16 * 1024)
    pub fn k16() -> Self {
        Self(16384)
    }
    /// Pieces in 32 KibiBytes (32 * 1024)
    pub fn k32() -> Self {
        Self(32768)
    }
    /// Pieces in 64 KibiBytes (64 * 1024)
    pub fn k64() -> Self {
        Self(65536)
    }
    /// Pieces in 128 KibiBytes (128 * 1024)
    pub fn k128() -> Self {
        Self(131072)
    }
    /// Pieces in 256 KibiBytes (256 * 1024)
    pub fn k256() -> Self {
        Self(262144)
    }
    /// Pieces in 512 KibiBytes (512 * 1024)
    pub fn k512() -> Self {
        Self(524288)
    }
    /// Pieces in 1 MebiBytes (1 * 1024 * 1024)
    pub fn m1() -> Self {
        Self(1048576)
    }
    /// Pieces in 2 MebiBytes (2 * 1024 * 1024)
    pub fn m2() -> Self {
        Self(2097152)
    }
    /// Pieces in 4 MebiBytes (4 * 1024 * 1024)
    pub fn m4() -> Self {
        Self(4194304)
    }
    /// Pieces in 8 MebiBytes (8 * 1024 * 1024)
    pub fn m8() -> Self {
        Self(8388608)
    }
    /// Pieces in 16 MebiBytes (16 * 1024 * 1024)
    pub fn m16() -> Self {
        Self(16777216)
    }
    /// Pieces in 32 MebiBytes (32 * 1024 * 1024)
    pub fn m32() -> Self {
        Self(33554432)
    }
    /// Pieces in 64 MebiBytes (64 * 1024 * 1024)
    pub fn m64() -> Self {
        Self(67108864)
    }
    /// Pieces in 128 MebiBytes (128 * 1024 * 1024)
    pub fn m128() -> Self {
        Self(134217728)
    }
    /// Pieces in 256 MebiBytes (256 * 1024 * 1024)
    pub fn m256() -> Self {
        Self(268435456)
    }
}
