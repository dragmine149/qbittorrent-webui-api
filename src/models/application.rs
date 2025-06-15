use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Torrent info resposne object
#[derive(Debug, Deserialize)]
pub struct TorrentInfo {
    /// Time (Unix Epoch) when the torrent was added to the client
    pub added_on: i64,
    /// Amount of data left to download (bytes)
    pub amount_left: i64,
    /// Whether this torrent is managed by Automatic Torrent Management
    pub auto_tmm: bool,
    /// Percentage of file pieces currently available
    pub availability: f64,
    /// Category of the torrent
    pub category: String,
    /// Amount of transfer data completed (bytes)
    pub completed: i64,
    /// Time (Unix Epoch) when the torrent completed
    pub completion_on: i64,
    /// Absolute path of torrent content (root path for multifile torrents, absolute file path for singlefile torrents)
    pub content_path: String,
    /// Torrent download speed limit (bytes/s). -1 if unlimited.
    pub dl_limit: i64,
    /// Torrent download speed (bytes/s)
    pub dlspeed: i64,
    /// Amount of data downloaded
    pub downloaded: i64,
    /// Amount of data downloaded this session
    pub downloaded_session: i64,
    /// Torrent ETA (seconds)
    pub eta: i64,
    /// True if first last piece are prioritized
    pub f_l_piece_prio: bool,
    /// True if force start is enabled for this torrent
    pub force_start: bool,
    /// Torrent hash
    pub hash: Option<String>,
    /// True if torrent is from a private tracker (added in 5.0.0)
    ///
    /// NOTE: Documetaion is wrong filed name is "private" not "isPrivate"
    pub private: bool,
    /// Last time (Unix Epoch) when a chunk was downloaded/uploaded
    pub last_activity: i64,
    /// Magnet URI corresponding to this torrent
    pub magnet_uri: String,
    /// Maximum share ratio until torrent is stopped from seeding/uploading
    pub max_ratio: f32,
    /// Maximum seeding time (seconds) until torrent is stopped from seeding
    pub max_seeding_time: i64,
    /// Torrent name
    pub name: String,
    /// Number of seeds in the swarm
    pub num_complete: i64,
    /// Number of leechers in the swarm
    pub num_incomplete: i64,
    /// Number of leechers connected to
    pub num_leechs: i64,
    /// Number of seeds connected to
    pub num_seeds: i64,
    /// Torrent priority. Returns -1 if queuing is disabled or torrent is in seed mode
    pub priority: i64,
    /// Torrent progress (percentage/100)
    pub progress: f32,
    /// Torrent share ratio. Max ratio value: 9999.
    pub ratio: f32,
    /// TODO (what is different from max_ratio?)
    pub ratio_limit: f32,
    /// Time until the next tracker reannounce
    pub reannounce: i64,
    /// Path where this torrent's data is stored
    pub save_path: String,
    /// Torrent elapsed time while complete (seconds)
    pub seeding_time: i64,
    /// TODO (what is different from max_seeding_time?) seeding_time_limit is a per torrent setting, when Automatic Torrent Management is disabled, furthermore then max_seeding_time is set to seeding_time_limit for this torrent. If Automatic Torrent Management is enabled, the value is -2. And if max_seeding_time is unset it have a default value -1.
    pub seeding_time_limit: i64,
    /// Time (Unix Epoch) when this torrent was last seen complete
    pub seen_complete: i64,
    /// True if sequential download is enabled
    pub seq_dl: bool,
    /// Total size (bytes) of files selected for download
    pub size: i64,
    /// Torrent state. See table here below for the possible values
    pub state: String,
    /// True if super seeding is enabled
    pub super_seeding: bool,
    /// Comma-concatenated tag list of the torrent
    pub tags: String,
    /// Total active time (seconds)
    pub time_active: i64,
    /// Total size (bytes) of all file in this torrent (including unselected ones)
    pub total_size: i64,
    /// The first tracker with working status. Returns empty string if no tracker is working.
    pub tracker: String,
    /// Torrent upload speed limit (bytes/s). -1 if unlimited.
    pub up_limit: i64,
    /// Amount of data uploaded
    pub uploaded: i64,
    /// Amount of data uploaded this session
    pub uploaded_session: i64,
    /// Torrent upload speed (bytes/s)
    pub upspeed: i64,
}

/// Build info resposne data object.
#[derive(Debug, Deserialize)]
pub struct BuildInfo {
    /// QT version
    pub qt: String,
    /// libtorrent version
    pub libtorrent: String,
    /// Boost version
    pub boost: String,
    /// OpenSSL version
    pub openssl: String,
    /// Application bitness (e.g. 64-bit)
    pub bitness: u8,
}

/// Preferences resposne data object.
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
    pub scheduler_days: Option<SchedulerTime>,
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

/// Scan dir types
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

/// Ratio actions
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum RatioAct {
    PauseTorrent = 0,
    RemoveTorrent = 1,
}

/// Bittorrent protocols
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum BittorrentProtocol {
    Tcpμtp = 0,
    Tcp = 1,
    MicroTransportProtocol = 2,
}

/// Scheduler
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum SchedulerTime {
    /// Every day
    Day = 0,
    /// Every Weekday
    Weekday = 1,
    /// Every Weekend
    Weekend = 2,
    /// Every Monday
    Monday = 3,
    /// Every Tuesday
    Tuesday = 4,
    /// Every Wednesday
    Wednesday = 5,
    /// Every Thursday
    Thursday = 6,
    /// Every Friday
    Friday = 7,
    /// Every Saturday
    Saturday = 8,
    /// Every Sunday
    Sunday = 9,
}

/// Encryption states
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Encryption {
    Prefer = 0,
    ForceOn = 1,
    ForceOff = 2,
}

/// Proxy types states
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

/// Dyndns servcice types
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum DyndnsService {
    Dydns = 0,
    Noip = 1,
}

/// Upload choking algorithm
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadChokingAlgorithm {
    RoundRobin = 0,
    FastestUpload = 1,
    AntiLeech = 2,
}

/// Upload slots behavior
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UploadSlotsBehavior {
    Fixed = 0,
    UploadRate = 1,
}

/// Mix mode UTP / TCP
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum UtpTcpMixedMode {
    PreferTcp = 0,
    PeerProportional = 1,
}
