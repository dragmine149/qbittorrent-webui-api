use crate::error::Error;

impl super::Client {
    pub async fn torrent_peers_data(&self, _hash: &str, _rid: usize) -> Result<Vec<()>, Error> {
        todo!("Not added. Documentaion missing!")
    }
}
