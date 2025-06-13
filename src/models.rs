use std::collections::HashMap;

use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    pub added_on: isize,
    pub amount_left: isize,
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: isize,
    pub completion_on: isize,
    pub content_path: String,
    pub dl_limit: isize,
    pub dlspeed: isize,
    pub downloaded: isize,
    pub downloaded_session: isize,
    pub eta: isize,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub hash: String,
    pub private: bool, // Documetaion is wrong filed name is "private" not "isPrivate"
    pub last_activity: isize,
    pub magnet_uri: String,
    pub max_ratio: f32,
    pub max_seeding_time: isize,
    pub name: String,
    pub num_complete: isize,
    pub num_incomplete: isize,
    pub num_leechs: isize,
    pub priority: isize,
    pub progress: f32,
    pub ratio: f32,
    pub ratio_limit: f32,
    pub reannounce: isize,
    pub save_path: String,
    pub seeding_time: isize,
    pub seeding_time_limit: isize,
    pub seen_complete: isize,
    pub seq_dl: bool,
    pub size: isize,
    pub state: String,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: isize,
    pub total_size: isize,
    pub tracker: String,
    pub up_limit: isize,
    pub uploaded: isize,
    pub uploaded_session: isize,
    pub upspeed: isize,
}

#[derive(Debug, Deserialize)]
pub struct TorrentProperties {
    pub save_path: String,
    pub creation_date: isize,
    pub piece_size: isize,
    pub comment: String,
    pub total_wasted: isize,
    pub total_uploaded: isize,
    pub total_uploaded_session: isize,
    pub total_downloaded: isize,
    pub total_downloaded_session: isize,
    pub up_limit: isize,
    pub dl_limit: isize,
    pub time_elapsed: isize,
    pub seeding_time: isize,
    pub nb_connections: isize,
    pub nb_connections_limit: isize,
    pub share_ratio: f32,
    pub addition_date: isize,
    pub completion_date: isize,
    pub created_by: String,
    pub dl_speed_avg: isize,
    pub dl_speed: isize,
    pub eta: isize,
    pub last_seen: isize,
    pub peers: isize,
    pub peers_total: isize,
    pub pieces_have: isize,
    pub pieces_num: isize,
    pub reannounce: isize,
    pub seeds: isize,
    pub seeds_total: isize,
    pub total_size: isize,
    pub up_speed_avg: isize,
    pub up_speed: isize,
    pub private: bool,
}

#[derive(Debug, Deserialize)]
pub struct TorrentTracker {
    pub url: String,
    pub status: isize,
    pub tier: isize,
    pub num_peers: isize,
    pub num_seeds: isize,
    pub num_leeches: isize,
    pub num_downloaded: isize,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct TorrentWebSeed {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TorrentContent {
    pub index: usize,
    pub name: String,
    pub size: usize,
    pub progress: f32,
    pub priority: FilePriority,
    pub is_seed: Option<bool>,
    pub piece_range: Vec<usize>,
    pub availability: f32,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum FilePriority {
    DoNotDownload = 0,
    Normal = 1,
    High = 6,
    Maximal = 7,
}

#[derive(Debug, Deserialize)]
pub struct LogItem {
    pub id: usize,
    pub message: String,
    pub timestamp: usize,
    #[serde(rename = "type")]
    pub log_type: LogType,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum LogType {
    Normal = 1,
    Info = 2,
    Warning = 4,
    Critical = 8,
}

#[derive(Debug, Deserialize)]
pub struct LogPeers {
    pub id: usize,
    pub ip: String,
    pub timestamp: usize,
    pub blocked: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferInfo {
    pub dl_info_speed: isize,
    pub dl_info_data: isize,
    pub up_info_speed: isize,
    pub up_info_data: isize,
    pub dl_rate_limit: isize,
    pub dht_nodes: isize,
    pub connection_status: ConnectionStatus,
    // pub queueing: Option<bool>, Cant find this in the API?
    pub last_external_address_v4: String, // This was not in the documentation!
    pub last_external_address_v6: String, // This was not in the documentation!
}

#[derive(Debug, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}

#[derive(Debug, Deserialize)]
pub struct MainData {
    pub rid: usize,
    pub full_update: Option<bool>,
    pub torrents: Option<HashMap<String, MainDataTorrentInfo>>,
    pub torrents_removed: Option<Vec<String>>,
    pub categories: Option<HashMap<String, Category>>,
    pub categories_removed: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub tags_removed: Option<Vec<String>>,
    pub server_state: Option<ServerStatus>,
    pub trackers: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub name: String,
    #[serde(rename = "savePath")]
    pub save_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerStatus {
    pub alltime_dl: usize,
    pub alltime_ul: usize,
    pub average_time_queue: usize,
    pub connection_status: ConnectionStatus,
    pub dht_nodes: usize,
    pub dl_info_data: usize,
    pub dl_info_speed: usize,
    pub dl_rate_limit: usize,
    pub free_space_on_disk: usize,
    pub global_ratio: String, // is in format of float
    pub last_external_address_v4: String,
    pub last_external_address_v6: String,
    pub queued_io_jobs: usize,
    pub queueing: bool,
    pub read_cache_hits: String,     // Is interger in format of string
    pub read_cache_overload: String, // Is interger in format of string
    pub refresh_interval: usize,
    pub total_buffers_size: usize,
    pub total_peer_connections: usize,
    pub total_queued_size: usize,
    pub total_wasted_session: usize,
    pub up_info_data: usize,
    pub up_info_speed: usize,
    pub up_rate_limit: usize,
    pub use_alt_speed_limits: bool,
    pub use_subcategories: bool,
    pub write_cache_overload: String, // Is interger in format of string
}

// same as TorrentInfo just without hash
#[derive(Debug, Deserialize)]
pub struct MainDataTorrentInfo {
    pub added_on: isize,
    pub amount_left: isize,
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: isize,
    pub completion_on: isize,
    pub content_path: String,
    pub dl_limit: isize,
    pub dlspeed: isize,
    pub downloaded: isize,
    pub downloaded_session: isize,
    pub eta: isize,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub private: bool, // Documetaion is wrong filed name is "private" not "isPrivate"
    pub last_activity: isize,
    pub magnet_uri: String,
    pub max_ratio: f32,
    pub max_seeding_time: isize,
    pub name: String,
    pub num_complete: isize,
    pub num_incomplete: isize,
    pub num_leechs: isize,
    pub priority: isize,
    pub progress: f32,
    pub ratio: f32,
    pub ratio_limit: f32,
    pub reannounce: isize,
    pub save_path: String,
    pub seeding_time: isize,
    pub seeding_time_limit: isize,
    pub seen_complete: isize,
    pub seq_dl: bool,
    pub size: isize,
    pub state: String,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: isize,
    pub total_size: isize,
    pub tracker: String,
    pub up_limit: isize,
    pub uploaded: isize,
    pub uploaded_session: isize,
    pub upspeed: isize,
}

// The API is incomplit on this and the structer is take from the responses
#[derive(Debug, Deserialize)]
pub struct PeersData {
    pub rid: usize,
    pub full_update: Option<bool>,
    pub show_flags: Option<bool>,
    pub peers: Option<HashMap<String, PeerData>>,
    pub peers_removed: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PeerData {
    pub client: Option<String>,
    pub connection: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub dl_speed: Option<usize>,
    pub downloaded: Option<usize>,
    pub files: Option<String>,
    pub flags: Option<String>,
    pub flags_desc: Option<String>,
    pub ip: Option<String>,
    pub peer_id_client: Option<String>,
    pub port: Option<usize>,
    pub progress: Option<f32>,
    pub relevance: Option<f32>,
    pub up_speed: Option<usize>,
    pub uploaded: Option<usize>,
}
