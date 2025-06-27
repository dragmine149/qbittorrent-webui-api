use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Search {
    /// ID of the search job
    pub id: u64,
    /// Current status of the search job
    pub status: SearchStatus,
    /// Total number of results. If the status is Running this number may contineu to increase
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub enum SearchStatus {
    Running,
    Stopped,
}
