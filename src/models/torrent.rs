use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Generic torrent properties
#[derive(Debug, Deserialize)]
pub struct TorrentProperties {
    /// Torrent save path
    pub save_path: String,
    /// Torrent creation date (Unix timestamp)
    pub creation_date: i64,
    /// Torrent piece size (bytes)
    pub piece_size: i64,
    /// Torrent comment
    pub comment: String,
    /// Total data wasted for torrent (bytes)
    pub total_wasted: i64,
    /// Total data uploaded for torrent (bytes)
    pub total_uploaded: i64,
    /// Total data uploaded this session (bytes)
    pub total_uploaded_session: i64,
    /// Total data downloaded for torrent (bytes)
    pub total_downloaded: i64,
    /// Total data downloaded this session (bytes)
    pub total_downloaded_session: i64,
    /// Torrent upload limit (bytes/s)
    pub up_limit: i64,
    /// Torrent download limit (bytes/s)
    pub dl_limit: i64,
    /// Torrent elapsed time (seconds)
    pub time_elapsed: i64,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// Torrent connection count
    pub nb_connections: i64,
    /// Torrent connection count limit
    pub nb_connections_limit: i64,
    /// Torrent share ratio
    pub share_ratio: f32,
    /// When this torrent was added (unix timestamp)
    pub addition_date: i64,
    /// Torrent completion date (unix timestamp)
    pub completion_date: i64,
    /// Torrent creator
    pub created_by: String,
    /// Torrent average download speed (bytes/second)
    pub dl_speed_avg: i64,
    /// Torrent download speed (bytes/second)
    pub dl_speed: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// Last seen complete date (unix timestamp)
    pub last_seen: i64,
    /// Number of peers connected to
    pub peers: i64,
    /// Number of peers in the swarm
    pub peers_total: i64,
    /// Number of pieces owned
    pub pieces_have: i64,
    /// Number of pieces of the torrent
    pub pieces_num: i64,
    /// Number of seconds until the next announce
    pub reannounce: i64,
    /// Number of seeds connected to
    pub seeds: i64,
    /// Number of seeds in the swarm
    pub seeds_total: i64,
    /// Torrent total size (bytes)
    pub total_size: i64,
    /// Torrent average upload speed (bytes/second)
    pub up_speed_avg: i64,
    /// Torrent upload speed (bytes/second)
    pub up_speed: i64,
    /// True if torrent is from a private tracker
    pub private: bool,
}

/// Torrent tracker data object
#[derive(Debug, Deserialize)]
pub struct TorrentTracker {
    /// Tracker url
    pub url: String,
    /// Tracker status. See the table below for possible values
    pub status: i64,
    /// Tracker priority tier. Lower tier trackers are tried before higher tiers. Tier numbers are valid when `>= 0`, `< 0` is used as placeholder when `tier` does not exist for special entries (such as DHT).
    pub tier: i64,
    /// Number of peers for current torrent, as reported by the tracker
    pub num_peers: i64,
    /// Number of seeds for current torrent, asreported by the tracker
    pub num_seeds: i64,
    /// Number of leeches for current torrent, as reported by the tracker
    pub num_leeches: i64,
    /// Number of completed downloads for current torrent, as reported by the tracker
    pub num_downloaded: i64,
    /// Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
    pub msg: String,
}

/// Web seed data object
#[derive(Debug, Deserialize)]
pub struct TorrentWebSeed {
    /// Web seed URL
    pub url: String,
}

/// Torrent file/content data object
#[derive(Debug, Deserialize)]
pub struct TorrentContent {
    /// File index
    pub index: i64,
    /// File name (including relative path)
    pub name: String,
    /// File size (bytes)
    pub size: i64,
    /// File progress (percentage/100)
    pub progress: f64,
    /// File priority.
    pub priority: FilePriority,
    /// True if file is seeding/complete
    pub is_seed: Option<bool>,
    /// The first number is the starting piece index and the second number is the ending piece index (inclusive)
    pub piece_range: Vec<i64>,
    /// Percentage of file pieces currently available (percentage/100)
    pub availability: f64,
}

/// File priority enum
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FilePriority {
    /// Do not download
    DoNotDownload = 0,
    /// Normal priority
    Normal = 1,
    /// High priority
    High = 6,
    /// Maximal priority
    Maximal = 7,
}

/// Pices state
#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum PiecesState {
    NotDownloaded = 0,
    Downloading = 1,
    Downloaded = 2,
}
