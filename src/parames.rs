#[derive(Debug)]
pub struct TorrentListParams {
    pub filter: Option<TorrentState>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<TorrentSort>,
    pub reverse: bool,
    pub limit: Option<isize>,
    pub offset: Option<isize>,
    pub hashes: Option<Vec<String>>,
}

impl TorrentListParams {
    pub fn deafult() -> Self {
        Self {
            filter: None,
            category: None,
            tag: None,
            sort: None,
            reverse: false,
            limit: None,
            offset: None,
            hashes: None,
        }
    }
}

#[derive(Debug)]
pub enum TorrentState {
    All,
    Downloading,
    Seeding,
    Completed,
    Stopped,
    Active,
    Inactive,
    Running,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored,
}

impl ToString for TorrentState {
    fn to_string(&self) -> String {
        match self {
            Self::All => String::from("all"),
            Self::Downloading => String::from("downloading"),
            Self::Seeding => String::from("seeding"),
            Self::Completed => String::from("completed"),
            Self::Stopped => String::from("stopped"),
            Self::Active => String::from("active"),
            Self::Inactive => String::from("inactive"),
            Self::Running => String::from("running"),
            Self::Stalled => String::from("stalled"),
            Self::StalledUploading => String::from("stalled_uploading"),
            Self::StalledDownloading => String::from("stalled_downloading"),
            Self::Errored => String::from("errored"),
        }
    }
}

#[derive(Debug)]
pub enum TorrentSort {
    AddedOn,
    AmountLeft,
    AutoTmm,
    Availability,
    Category,
    Completed,
    CompletionOn,
    ContentPath,
    DlLimit,
    Dlspeed,
    Downloaded,
    DownloadedSession,
    ETA,
    FLPiecePrio,
    ForceStart,
    Hash,
    Private,
    LastActivity,
    MagnetUri,
    MaxRatio,
    MaxSeedingTime,
    Name,
    NumComplete,
    NumIncomplete,
    NumLeechs,
    NumSeeds,
    Priority,
    Progress,
    Ratio,
    RatioLimit,
    Reannounce,
    SavePath,
    SeedingTime,
    SeedingTimeLimit,
    SeenComplete,
    SeqDl,
    Size,
    State,
    SuperSeeding,
    Tags,
    TimeActive,
    TotalSize,
    Tracker,
    UpLimit,
    Uploaded,
    UploadedSession,
    Upspeed,
}

impl ToString for TorrentSort {
    fn to_string(&self) -> String {
        match self {
            Self::AddedOn => String::from("added_on"),
            Self::AmountLeft => String::from("amount_left"),
            Self::AutoTmm => String::from("auto_tmm"),
            Self::Availability => String::from("availability"),
            Self::Category => String::from("category"),
            Self::Completed => String::from("completed"),
            Self::CompletionOn => String::from("completion_on"),
            Self::ContentPath => String::from("content_path"),
            Self::DlLimit => String::from("dl_limit"),
            Self::Dlspeed => String::from("dlspeed"),
            Self::Downloaded => String::from("downloaded"),
            Self::DownloadedSession => String::from("downloaded_session"),
            Self::ETA => String::from("eta"),
            Self::FLPiecePrio => String::from("f_l_piece_prio"),
            Self::ForceStart => String::from("force_start"),
            Self::Hash => String::from("hash"),
            Self::Private => String::from("private"),
            Self::LastActivity => String::from("last_activity"),
            Self::MagnetUri => String::from("magnet_uri"),
            Self::MaxRatio => String::from("max_ratio"),
            Self::MaxSeedingTime => String::from("max_seeding_time"),
            Self::Name => String::from("name"),
            Self::NumComplete => String::from("num_complete"),
            Self::NumIncomplete => String::from("num_incomplete"),
            Self::NumLeechs => String::from("num_leechs"),
            Self::NumSeeds => String::from("num_seeds"),
            Self::Priority => String::from("priority"),
            Self::Progress => String::from("progress"),
            Self::Ratio => String::from("ratio"),
            Self::RatioLimit => String::from("ratio_limit"),
            Self::Reannounce => String::from("reannounce"),
            Self::SavePath => String::from("save_path"),
            Self::SeedingTime => String::from("seeding_time"),
            Self::SeedingTimeLimit => String::from("seeding_time_limit"),
            Self::SeenComplete => String::from("seen_complete"),
            Self::SeqDl => String::from("seq_dl"),
            Self::Size => String::from("size"),
            Self::State => String::from("state"),
            Self::SuperSeeding => String::from("super_seeding"),
            Self::Tags => String::from("tags"),
            Self::TimeActive => String::from("time_active"),
            Self::TotalSize => String::from("total_size"),
            Self::Tracker => String::from("tracker"),
            Self::UpLimit => String::from("up_limit"),
            Self::Uploaded => String::from("uploaded"),
            Self::UploadedSession => String::from("uploaded_session"),
            Self::Upspeed => String::from("upspeed"),
        }
    }
}

#[derive(Debug)]
pub struct TorrentAddUrls {
    pub urls: Vec<String>,
    pub savepath: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub skip_checking: bool,
    pub paused: bool,
    pub root_folder: Option<String>,
    pub rename: Option<String>,
    pub up_limit: Option<usize>,
    pub dl_limit: Option<usize>,
    pub ratio_limit: Option<f32>,
    pub seeding_time_limit: Option<usize>,
    pub auto_tmm: bool,
    pub sequential_download: bool,
    pub first_last_piece_prio: bool,
}

impl TorrentAddUrls {
    pub fn deafult(urls: Vec<String>) -> Self {
        Self {
            urls,
            savepath: None,
            category: None,
            tags: None,
            skip_checking: false,
            paused: false,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            ratio_limit: None,
            seeding_time_limit: None,
            auto_tmm: false,
            sequential_download: false,
            first_last_piece_prio: false,
        }
    }
}

#[derive(Debug)]
pub struct TorrentTrackersList {
    pub hash: String,
    pub urls: Vec<String>,
}

#[derive(Debug)]
pub struct TorrentTrackersEdit {
    pub hash: String,
    pub orig_url: String,
    pub new_url: String,
}

#[derive(Debug)]
pub struct TorrentAddPeers {
    pub hashes: Vec<String>,
    pub peers: Vec<String>,
}
