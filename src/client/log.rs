use crate::{
    error::Error,
    models::{LogItem, LogPeers, LogType},
};

impl super::Api {
    pub async fn get_log(
        &self,
        last_known_id: Option<i64>,
        log_types: Option<Vec<LogType>>,
    ) -> Result<Vec<LogItem>, Error> {
        let mut url = self._build_url("api/v2/log/main").await?;

        let mut query = url.query_pairs_mut();
        if let Some(last_known_id) = last_known_id {
            query.append_pair("last_known_id", &last_known_id.to_string());
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
        if !normal {
            query.append_pair("normal", &false.to_string());
        }
        if !info {
            query.append_pair("info", &false.to_string());
        }
        if !warning {
            query.append_pair("warning", &false.to_string());
        }
        if !critical {
            query.append_pair("critical", &false.to_string());
        }
        drop(query);

        let log = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<LogItem>>()
            .await?;

        Ok(log)
    }

    pub async fn get_log_peer(&self, last_known_id: Option<i64>) -> Result<Vec<LogPeers>, Error> {
        let mut url = self._build_url("api/v2/log/peers").await?;
        if let Some(id) = last_known_id {
            url.set_query(Some(&format!("last_known_id={}", id)));
        }

        let log = self
            .http_client
            .get(url)
            .send()
            .await?
            .json::<Vec<LogPeers>>()
            .await?;

        Ok(log)
    }
}
