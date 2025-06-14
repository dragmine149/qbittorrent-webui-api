use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    pub added_on: i64,
    pub amount_left: i64,
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: i64,
    pub completion_on: i64,
    pub content_path: String,
    pub dl_limit: i64,
    pub dlspeed: i64,
    pub downloaded: i64,
    pub downloaded_session: i64,
    pub eta: i64,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub hash: String,
    pub private: bool, // Documetaion is wrong filed name is "private" not "isPrivate"
    pub last_activity: i64,
    pub magnet_uri: String,
    pub max_ratio: f32,
    pub max_seeding_time: i64,
    pub name: String,
    pub num_complete: i64,
    pub num_incomplete: i64,
    pub num_leechs: i64,
    pub priority: i64,
    pub progress: f32,
    pub ratio: f32,
    pub ratio_limit: f32,
    pub reannounce: i64,
    pub save_path: String,
    pub seeding_time: i64,
    pub seeding_time_limit: i64,
    pub seen_complete: i64,
    pub seq_dl: bool,
    pub size: i64,
    pub state: String,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: i64,
    pub total_size: i64,
    pub tracker: String,
    pub up_limit: i64,
    pub uploaded: i64,
    pub uploaded_session: i64,
    pub upspeed: i64,
}

#[derive(Debug, Deserialize)]
pub struct TorrentProperties {
    pub save_path: String,
    pub creation_date: i64,
    pub piece_size: i64,
    pub comment: String,
    pub total_wasted: i64,
    pub total_uploaded: i64,
    pub total_uploaded_session: i64,
    pub total_downloaded: i64,
    pub total_downloaded_session: i64,
    pub up_limit: i64,
    pub dl_limit: i64,
    pub time_elapsed: i64,
    pub seeding_time: i64,
    pub nb_connections: i64,
    pub nb_connections_limit: i64,
    pub share_ratio: f32,
    pub addition_date: i64,
    pub completion_date: i64,
    pub created_by: String,
    pub dl_speed_avg: i64,
    pub dl_speed: i64,
    pub eta: i64,
    pub last_seen: i64,
    pub peers: i64,
    pub peers_total: i64,
    pub pieces_have: i64,
    pub pieces_num: i64,
    pub reannounce: i64,
    pub seeds: i64,
    pub seeds_total: i64,
    pub total_size: i64,
    pub up_speed_avg: i64,
    pub up_speed: i64,
    pub private: bool,
}

#[derive(Debug, Deserialize)]
pub struct TorrentTracker {
    pub url: String,
    pub status: i64,
    pub tier: i64,
    pub num_peers: i64,
    pub num_seeds: i64,
    pub num_leeches: i64,
    pub num_downloaded: i64,
    pub msg: String,
}

#[derive(Debug, Deserialize)]
pub struct TorrentWebSeed {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TorrentContent {
    pub index: i64,
    pub name: String,
    pub size: i64,
    pub progress: f32,
    pub priority: FilePriority,
    pub is_seed: Option<bool>,
    pub piece_range: Vec<i64>,
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
    pub id: i64,
    pub message: String,
    pub timestamp: i64,
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
    pub id: i64,
    pub ip: String,
    pub timestamp: i64,
    pub blocked: bool,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferInfo {
    pub dl_info_speed: i64,
    pub dl_info_data: i64,
    pub up_info_speed: i64,
    pub up_info_data: i64,
    pub dl_rate_limit: i64,
    pub dht_nodes: i64,
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
    pub rid: i64,
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
    pub alltime_dl: i64,
    pub alltime_ul: i64,
    pub average_time_queue: i64,
    pub connection_status: ConnectionStatus,
    pub dht_nodes: i64,
    pub dl_info_data: i64,
    pub dl_info_speed: i64,
    pub dl_rate_limit: i64,
    pub free_space_on_disk: i64,
    pub global_ratio: String, // is in format of float
    pub last_external_address_v4: String,
    pub last_external_address_v6: String,
    pub queued_io_jobs: i64,
    pub queueing: bool,
    pub read_cache_hits: String,     // Is interger in format of string
    pub read_cache_overload: String, // Is interger in format of string
    pub refresh_interval: i64,
    pub total_buffers_size: i64,
    pub total_peer_connections: i64,
    pub total_queued_size: i64,
    pub total_wasted_session: i64,
    pub up_info_data: i64,
    pub up_info_speed: i64,
    pub up_rate_limit: i64,
    pub use_alt_speed_limits: bool,
    pub use_subcategories: bool,
    pub write_cache_overload: String, // Is interger in format of string
}

// same as TorrentInfo just without hash
#[derive(Debug, Deserialize)]
pub struct MainDataTorrentInfo {
    pub added_on: i64,
    pub amount_left: i64,
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: i64,
    pub completion_on: i64,
    pub content_path: String,
    pub dl_limit: i64,
    pub dlspeed: i64,
    pub downloaded: i64,
    pub downloaded_session: i64,
    pub eta: i64,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub private: bool, // Documetaion is wrong filed name is "private" not "isPrivate"
    pub last_activity: i64,
    pub magnet_uri: String,
    pub max_ratio: f32,
    pub max_seeding_time: i64,
    pub name: String,
    pub num_complete: i64,
    pub num_incomplete: i64,
    pub num_leechs: i64,
    pub priority: i64,
    pub progress: f32,
    pub ratio: f32,
    pub ratio_limit: f32,
    pub reannounce: i64,
    pub save_path: String,
    pub seeding_time: i64,
    pub seeding_time_limit: i64,
    pub seen_complete: i64,
    pub seq_dl: bool,
    pub size: i64,
    pub state: String,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: i64,
    pub total_size: i64,
    pub tracker: String,
    pub up_limit: i64,
    pub uploaded: i64,
    pub uploaded_session: i64,
    pub upspeed: i64,
}

// The API is incomplit on this and the structer is take from the responses
#[derive(Debug, Deserialize)]
pub struct PeersData {
    pub rid: i64,
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
    pub dl_speed: Option<i64>,
    pub downloaded: Option<i64>,
    pub files: Option<String>,
    pub flags: Option<String>,
    pub flags_desc: Option<String>,
    pub ip: Option<String>,
    pub peer_id_client: Option<String>,
    pub port: Option<i64>,
    pub progress: Option<f32>,
    pub relevance: Option<f32>,
    pub up_speed: Option<i64>,
    pub uploaded: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BuildInfo {
    pub qt: String,
    pub libtorrent: String,
    pub boost: String,
    pub openssl: String,
    pub bitness: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Preferences {
    /// Currently selected language (e.g. en_GB for English)
    pub locale: Option<String>,
    /// True if a subfolder should be created when adding a torrent
    pub create_subfolder_enabled: Option<bool>,
    /// True if torrents should be added in a Paused state
    pub start_paused_enabled: Option<bool>,
    /// TODO
    pub auto_delete_mode: Option<i64>,
    /// True if disk space should be pre-allocated for all files
    pub preallocate_all: Option<bool>,
    /// True if ".!qB" should be appended to incomplete files
    pub incomplete_files_ext: Option<bool>,
    /// True if Automatic Torrent Management is enabled by default
    pub auto_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when its Category changes
    pub torrent_changed_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when the default save path changes
    pub save_path_changed_tmm_enabled: Option<bool>,
    /// True if torrent should be relocated when its Category's save path changes
    pub category_changed_tmm_enabled: Option<bool>,
    /// Default save path for torrents, separated by slashes
    pub save_path: Option<String>,
    /// True if folder for incomplete torrents is enabled
    pub temp_path_enabled: Option<bool>,
    /// Path for incomplete torrents, separated by slashes
    pub temp_path: Option<String>,
    // Property: directory to watch for torrent files, value: where torrents loaded from this directory should be downloaded to (see list of possible values below). Slashes are used as path separators; multiple key/value pairs can be specified
    pub scan_dirs: Option<HashMap<String, ScanDir>>,
    /// Path to directory to copy .torrent files to. Slashes are used as path separators
    pub export_dir: Option<String>,
    /// Path to directory to copy .torrent files of completed downloads to. Slashes are used as path separators
    pub export_dir_fin: Option<String>,
    /// True if e-mail notification should be enabled
    pub mail_notification_enabled: Option<bool>,
    /// e-mail where notifications should originate from
    pub mail_notification_sender: Option<String>,
    /// e-mail to send notifications to
    pub mail_notification_email: Option<String>,
    /// smtp server for e-mail notifications
    pub mail_notification_smtp: Option<String>,
    /// True if smtp server requires SSL connection
    pub mail_notification_ssl_enabled: Option<bool>,
    /// True if smtp server requires authentication
    pub mail_notification_auth_enabled: Option<bool>,
    /// Username for smtp authentication
    pub mail_notification_username: Option<String>,
    /// Password for smtp authentication
    pub mail_notification_password: Option<String>,
    /// True if external program should be run after torrent has finished downloading
    pub autorun_enabled: Option<bool>,
    /// Program path/name/arguments to run if `autorun_enabled` is enabled; path is separated by slashes; you can use `%f` and `%n` arguments, which will be expanded by qBittorent as path_to_torrent_file and torrent_name (from the GUI; not the .torrent file name) respectively
    pub autorun_program: Option<String>,
    /// True if torrent queuing is enabled
    pub queueing_enabled: Option<bool>,
    /// Maximum number of active simultaneous downloads
    pub max_active_downloads: Option<i64>,
    /// Maximum number of active simultaneous downloads and uploads
    pub max_active_torrents: Option<i64>,
    /// Maximum number of active simultaneous uploads
    pub max_active_uploads: Option<i64>,
    /// If true torrents w/o any activity (stalled ones) will not be counted towards `max_active_*` limits; see dont_count_slow_torrents for more information
    pub dont_count_slow_torrents: Option<bool>,
    /// Download rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_dl_rate_threshold: Option<i64>,
    /// Upload rate in KiB/s for a torrent to be considered "slow"
    pub slow_torrent_ul_rate_threshold: Option<i64>,
    /// Seconds a torrent should be inactive before considered "slow"
    pub slow_torrent_inactive_timer: Option<i64>,
    /// True if share ratio limit is enabled
    pub max_ratio_enabled: Option<bool>,
    /// Get the global share ratio limit
    pub max_ratio: Option<f32>,
    /// Action performed when a torrent reaches the maximum share ratio. See list of possible values here below.
    pub max_ratio_act: Option<RatioAct>,
    /// Port for incoming connections
    pub listen_port: Option<i64>,
    /// True if UPnP/NAT-PMP is enabled
    pub upnp: Option<bool>,
    /// True if the port is randomly selected
    pub random_port: Option<bool>,
    /// Global download speed limit in KiB/s; -1 means no limit is applied
    pub dl_limit: Option<i64>,
    /// Global upload speed limit in KiB/s; -1 means no limit is applied
    pub up_limit: Option<i64>,
    /// Maximum global number of simultaneous connections
    pub max_connec: Option<i64>,
    /// Maximum number of simultaneous connections per torrent
    pub max_connec_per_torrent: Option<i64>,
    /// Maximum number of upload slots
    pub max_uploads: Option<i64>,
    /// Maximum number of upload slots per torrent
    pub max_uploads_per_torrent: Option<i64>,
    /// Timeout in seconds for a stopped announce request to trackers
    pub stop_tracker_timeout: Option<i64>,
    /// True if the advanced libtorrent option piece_extent_affinity is enabled
    pub enable_piece_extent_affinity: Option<bool>,
    /// Bittorrent Protocol to use (see list of possible values below)
    pub bittorrent_protocol: Option<BittorrentProtocol>,
    /// True if [du]l_limit should be applied to uTP connections; this option is only available in qBittorent built against libtorrent version 0.16.X and higher
    pub limit_utp_rate: Option<bool>,
    /// True if [du]l_limit should be applied to estimated TCP overhead (service data: e.g. packet headers)
    pub limit_tcp_overhead: Option<bool>,
    /// True if [du]l_limit should be applied to peers on the LAN
    pub limit_lan_peers: Option<bool>,
    /// Alternative global download speed limit in KiB/s
    pub alt_dl_limit: Option<i64>,
    /// Alternative global upload speed limit in KiB/s
    pub alt_up_limit: Option<i64>,
    /// True if alternative limits should be applied according to schedule
    pub scheduler_enabled: Option<bool>,
    /// Scheduler starting hour
    pub schedule_from_hour: Option<i64>,
    /// Scheduler starting minute
    pub schedule_from_min: Option<i64>,
    /// Scheduler ending hour
    pub schedule_to_hour: Option<i64>,
    /// Scheduler ending minute
    pub schedule_to_min: Option<i64>,
    /// Scheduler days. See possible values here below
    pub scheduler_days: Option<SchedulerDay>,
    /// True if DHT is enabled
    pub dht: Option<bool>,
    /// True if PeX is enabled
    pub pex: Option<bool>,
    /// True if LSD is enabled
    pub lsd: Option<bool>,
    /// See list of possible values here below
    pub encryption: Option<Encryption>,
    /// If true anonymous mode will be enabled; read more here; this option is only available in qBittorent built against libtorrent version 0.16.X and higher
    pub anonymous_mode: Option<bool>,
    /// See list of possible values here below
    pub proxy_type: Option<ProxyType>,
    /// Proxy IP address or domain name
    pub proxy_ip: Option<String>,
    /// Proxy port
    pub proxy_port: Option<i64>,
    /// True if peer and web seed connections should be proxified; this option will have any effect only in qBittorent built against libtorrent version 0.16.X and higher
    pub proxy_peer_connections: Option<bool>,
    /// True proxy requires authentication; doesn't apply to SOCKS4 proxies
    pub proxy_auth_enabled: Option<bool>,
    /// Username for proxy authentication
    pub proxy_username: Option<String>,
    /// Password for proxy authentication
    pub proxy_password: Option<String>,
    /// True if proxy is only used for torrents
    pub proxy_torrents_only: Option<bool>,
    /// True if external IP filter should be enabled
    pub ip_filter_enabled: Option<bool>,
    /// Path to IP filter file (.dat, .p2p, .p2b files are supported); path is separated by slashes
    pub ip_filter_path: Option<String>,
    /// True if IP filters are applied to trackers
    pub ip_filter_trackers: Option<bool>,
    /// Semicolon-separated list of domains to accept when performing Host header validation
    pub web_ui_domain_list: Option<String>,
    /// IP address to use for the WebUI
    pub web_ui_address: Option<String>,
    /// WebUI port
    pub web_ui_port: Option<i64>,
    /// True if UPnP is used for the WebUI port
    pub web_ui_upnp: Option<bool>,
    /// WebUI username
    pub web_ui_username: Option<String>,
    /// For API ≥ v2.3.0: Plaintext WebUI password, not readable, write-only. For API < v2.3.0: MD5 hash of WebUI password, hash is generated from the following string: username:Web UI Access:plain_text_web_ui_password
    pub web_ui_password: Option<String>,
    /// True if WebUI CSRF protection is enabled
    pub web_ui_csrf_protection_enabled: Option<bool>,
    /// True if WebUI clickjacking protection is enabled
    pub web_ui_clickjacking_protection_enabled: Option<bool>,
    /// True if WebUI cookie Secure flag is enabled
    pub web_ui_secure_cookie_enabled: Option<bool>,
    /// Maximum number of authentication failures before WebUI access ban
    pub web_ui_max_auth_fail_count: Option<i64>,
    /// WebUI access ban duration in seconds
    pub web_ui_ban_duration: Option<i64>,
    /// Seconds until WebUI is automatically signed off
    pub web_ui_session_timeout: Option<i64>,
    /// True if WebUI host header validation is enabled
    pub web_ui_host_header_validation_enabled: Option<bool>,
    /// True if authentication challenge for loopback address (127.0.0.1) should be disabled
    pub bypass_local_auth: Option<bool>,
    /// True if webui authentication should be bypassed for clients whose ip resides within (at least) one of the subnets on the whitelist
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,
    /// (White)list of ipv4/ipv6 subnets for which webui authentication should be bypassed; list entries are separated by commas
    pub bypass_auth_subnet_whitelist: Option<String>,
    /// True if an alternative WebUI should be used
    pub alternative_webui_enabled: Option<bool>,
    /// File path to the alternative WebUI
    pub alternative_webui_path: Option<String>,
    /// True if WebUI HTTPS access is enabled
    pub use_https: Option<bool>,
    /// For API < v2.0.1: SSL keyfile contents (this is a not a path)
    pub ssl_key: Option<String>,
    /// For API < v2.0.1: SSL certificate contents (this is a not a path)
    pub ssl_cert: Option<String>,
    /// For API ≥ v2.0.1: Path to SSL keyfile
    pub web_ui_https_key_path: Option<String>,
    /// For API ≥ v2.0.1: Path to SSL certificate
    pub web_ui_https_cert_path: Option<String>,
    /// True if server DNS should be updated dynamically
    pub dyndns_enabled: Option<bool>,
    /// See list of possible values here below
    pub dyndns_service: Option<DyndnsService>,
    /// Username for DDNS service
    pub dyndns_username: Option<String>,
    /// Password for DDNS service
    pub dyndns_password: Option<String>,
    /// Your DDNS domain name
    pub dyndns_domain: Option<String>,
    /// RSS refresh interval
    pub rss_refresh_interval: Option<i64>,
    /// Max stored articles per RSS feed
    pub rss_max_articles_per_feed: Option<i64>,
    /// Enable processing of RSS feeds
    pub rss_processing_enabled: Option<bool>,
    /// Enable auto-downloading of torrents from the RSS feeds
    pub rss_auto_downloading_enabled: Option<bool>,
    /// For API ≥ v2.5.1: Enable downloading of repack/proper Episodes
    pub rss_download_repack_proper_episodes: Option<bool>,
    /// For API ≥ v2.5.1: List of RSS Smart Episode Filters
    pub rss_smart_episode_filters: Option<String>,
    /// Enable automatic adding of trackers to new torrents
    pub add_trackers_enabled: Option<bool>,
    /// List of trackers to add to new torrent
    pub add_trackers: Option<String>,
    /// For API ≥ v2.5.1: Enable custom http headers
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,
    /// For API ≥ v2.5.1: List of custom http headers
    pub web_ui_custom_http_headers: Option<String>,
    /// True enables max seeding time
    pub max_seeding_time_enabled: Option<bool>,
    /// Number of minutes to seed a torrent
    pub max_seeding_time: Option<i64>,
    /// TODO
    pub announce_ip: Option<String>,
    /// True always announce to all tiers
    pub announce_to_all_tiers: Option<bool>,
    /// True always announce to all trackers in a tier
    pub announce_to_all_trackers: Option<bool>,
    /// Number of asynchronous I/O threads
    pub async_io_threads: Option<i64>,
    /// List of banned IPs
    #[serde(rename = "banned_IPs")]
    pub banned_ips: Option<String>,
    /// Outstanding memory when checking torrents in MiB
    pub checking_memory_use: Option<i64>,
    /// IP Address to bind to. Empty String means All addresses
    pub current_interface_address: Option<String>,
    /// Network Interface used
    pub current_network_interface: Option<String>,
    /// Disk cache used in MiB
    pub disk_cache: Option<i64>,
    /// Disk cache expiry interval in seconds
    pub disk_cache_ttl: Option<i64>,
    /// Port used for embedded tracker
    pub embedded_tracker_port: Option<i64>,
    /// True enables coalesce reads & writes
    pub enable_coalesce_read_write: Option<bool>,
    /// True enables embedded tracker
    pub enable_embedded_tracker: Option<bool>,
    /// True allows multiple connections from the same IP address
    pub enable_multi_connections_from_same_ip: Option<bool>,
    /// True enables os cache
    pub enable_os_cache: Option<bool>,
    /// True enables sending of upload piece suggestions
    pub enable_upload_suggestions: Option<bool>,
    /// File pool size
    pub file_pool_size: Option<i64>,
    /// Maximal outgoing port (0: Disabled)
    pub outgoing_ports_max: Option<i64>,
    /// Minimal outgoing port (0: Disabled)
    pub outgoing_ports_min: Option<i64>,
    /// True rechecks torrents on completion
    pub recheck_completed_torrents: Option<bool>,
    /// True resolves peer countries
    pub resolve_peer_countries: Option<bool>,
    /// Save resume data interval in min
    pub save_resume_data_interval: Option<i64>,
    /// Send buffer low watermark in KiB
    pub send_buffer_low_watermark: Option<i64>,
    /// Send buffer watermark in KiB
    pub send_buffer_watermark: Option<i64>,
    /// Send buffer watermark factor in percent
    pub send_buffer_watermark_factor: Option<i64>,
    /// Socket backlog size
    pub socket_backlog_size: Option<i64>,
    /// Upload choking algorithm used (see list of possible values below)
    pub upload_choking_algorithm: Option<UploadChokingAlgorithm>,
    /// Upload slots behavior used (see list of possible values below)
    pub upload_slots_behavior: Option<UploadSlotsBehavior>,
    /// UPnP lease duration (0: Permanent lease)
    pub upnp_lease_duration: Option<i64>,
    /// μTP-TCP mixed mode algorithm (see list of possible values below)
    pub utp_tcp_mixed_mode: Option<UtpTcpMixedMode>,
}

#[derive(Debug)]
pub enum ScanDir {
    MonitoredFolder,
    DefaultSavePath,
    OtherPath(String),
}

impl<'de> Deserialize<'de> for ScanDir {
    fn deserialize<D>(deserializer: D) -> Result<ScanDir, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Int(u8),
            Str(String),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Int(0) => Ok(ScanDir::MonitoredFolder),
            Helper::Int(1) => Ok(ScanDir::DefaultSavePath),
            Helper::Str(s) => Ok(ScanDir::OtherPath(s)),
            _ => Err(serde::de::Error::custom(
                "unexpected value, expected 0, 1, or a string (path)",
            )),
        }
    }
}

impl Serialize for ScanDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ScanDir::MonitoredFolder => serializer.serialize_u8(0),
            ScanDir::DefaultSavePath => serializer.serialize_i8(1),
            ScanDir::OtherPath(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RatioAct {
    PauseTorrent = 0,
    RemoveTorrent = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum BittorrentProtocol {
    Tcpμtp = 0,
    Tcp = 1,
    MicroTransportProtocol = 2,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SchedulerDay {
    Day = 0,
    Weekday = 1,
    Weekend = 2,
    Monday = 3,
    Tuesday = 4,
    Wednesday = 5,
    Thursday = 6,
    Friday = 7,
    Saturday = 8,
    Sunday = 9,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Encryption {
    Prefer = 0,
    ForceOn = 1,
    ForceOff = 2,
}

#[derive(Debug)]
pub enum ProxyType {
    Disabled,
    Other(String),
    HttpWithoutAuth,
    Socks5WithoutAuth,
    HttpWithAuth,
    Socks5WithAuth,
    Socks4WithoutAuth,
}

impl<'de> Deserialize<'de> for ProxyType {
    fn deserialize<D>(deserializer: D) -> Result<ProxyType, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Int(i8),
            Str(String),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Int(-1) => Ok(ProxyType::Disabled),
            Helper::Str(s) => Ok(ProxyType::Other(s)),
            Helper::Int(1) => Ok(ProxyType::HttpWithoutAuth),
            Helper::Int(2) => Ok(ProxyType::Socks5WithoutAuth),
            Helper::Int(3) => Ok(ProxyType::HttpWithAuth),
            Helper::Int(4) => Ok(ProxyType::Socks5WithAuth),
            Helper::Int(5) => Ok(ProxyType::Socks4WithoutAuth),
            _ => Err(serde::de::Error::custom(
                "unexpected value, expected 1 to 5, -1 or a string",
            )),
        }
    }
}

impl Serialize for ProxyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ProxyType::Disabled => serializer.serialize_i8(-1),
            ProxyType::HttpWithoutAuth => serializer.serialize_i8(1),
            ProxyType::Socks5WithoutAuth => serializer.serialize_i8(2),
            ProxyType::HttpWithAuth => serializer.serialize_i8(3),
            ProxyType::Socks5WithAuth => serializer.serialize_i8(4),
            ProxyType::Socks4WithoutAuth => serializer.serialize_i8(5),
            ProxyType::Other(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DyndnsService {
    Dydns = 0,
    Noip = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    Fixed = 0,
    UploadRate = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTcp = 0,
    PeerProportional = 1,
}
