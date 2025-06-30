use serde::Deserialize;

mod application;
mod log;
mod rss;
mod search;
mod sync;
mod torrent;
mod transfer;

pub use application::*;
pub use log::*;
pub use rss::*;
pub use search::*;
pub use sync::*;
pub use torrent::*;
pub use transfer::*;

/// Connection status of the Qbit application
#[derive(Debug, Deserialize)]
pub enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "firewalled")]
    Firewalled,
    #[serde(rename = "disconnected")]
    Disconnected,
}

#[derive(Clone)]
pub struct Credentials {
    username: String,
    password: String,
}

impl Credentials {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl ToString for Credentials {
    fn to_string(&self) -> String {
        format!("username={}&password={}", self.username, self.password)
    }
}
