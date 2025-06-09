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
    Category,
    ETA,
    Ratio,
    Tags,
    Dlspeed,
    Name,
    State,
}

impl ToString for TorrentSort {
    fn to_string(&self) -> String {
        match self {
            Self::AddedOn => String::from("added_on"),
            Self::Category => String::from("category"),
            Self::ETA => String::from("eta"),
            Self::Ratio => String::from("ratio"),
            Self::Tags => String::from("tags"),
            Self::Dlspeed => String::from("dlspeed"),
            Self::Name => String::from("name"),
            Self::State => String::from("state"),
        }
    }
}
