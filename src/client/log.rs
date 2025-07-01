use crate::{
    error::Error,
    models::{LogItem, LogPeers, LogType},
};

impl super::Api {
    /// Get log
    /// 
    /// Retrieves the main log of the qBittorrent application.
    /// 
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
    /// * `log_types` - List of desierd log types. (default: all)
    /// 
    pub async fn log(
        &self,
        last_known_id: Option<i64>,
        log_types: Option<Vec<LogType>>,
    ) -> Result<Vec<LogItem>, Error> {
        let mut query = vec![];
        if let Some(last_known_id) = last_known_id {
            query.push(("last_known_id", last_known_id.to_string()));
        }
        let mut normal = false;
        let mut info = false;
        let mut warning = false;
        let mut critical = false;
        if let Some(log_types) = log_types {
            for log_type in log_types {
                match log_type {
                    LogType::Normal => normal = true,
                    LogType::Info => info = true,
                    LogType::Warning => warning = true,
                    LogType::Critical => critical = true,
                }
            }
        }
        if normal {
            query.push(("normal", true.to_string()));
        }
        if info {
            query.push(("info", true.to_string()));
        }
        if warning {
            query.push(("warning", true.to_string()));
        }
        if critical {
            query.push(("critical", true.to_string()));
        }

        let log = self
            ._get("log/main")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<Vec<LogItem>>()
            .await?;

        Ok(log)
    }

    /// Get peer log
    /// 
    /// Retrieves the peer log of the qBittorrent application.
    /// 
    /// [official documentation](https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-5.0)#get-peer-log)
    ///
    /// # Arguments
    ///
    /// * `last_known_id` - Exclude messages with "message id" <= `last_known_id` (default: `-1`)
    /// 
    pub async fn peer_log(&self, last_known_id: Option<i64>) -> Result<Vec<LogPeers>, Error> {
        let mut query = vec![];
        if let Some(id) = last_known_id {
            query.push(("last_known_id", id));
        }

        let log = self
            ._get("log/peers")
            .await?
            .query(&query)
            .send()
            .await?
            .json::<Vec<LogPeers>>()
            .await?;

        Ok(log)
    }
}
