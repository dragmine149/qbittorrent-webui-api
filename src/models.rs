use serde::Deserialize;
use serde_repr::Deserialize_repr;

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

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum FilePriority {
    DoNotDownload = 0,
    Normal = 1,
    High = 6,
    Maximal = 7,
}

#[derive(Debug, Deserialize)]
pub struct LogPeers {
    pub id: usize,
    pub ip: String,
    pub timestamp: usize,
    pub blocked: bool,
    pub reason: String,
}
