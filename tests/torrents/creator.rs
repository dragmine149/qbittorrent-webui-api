use std::{
    fs,
    path::{Path, PathBuf},
};

use qbit::models::{TorrentCreator, TorrentCreatorBuilder};

use crate::login_default_client;

/// This test ensures that the API correctly deserialize the torrents the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_basic_torrent() {
    let client = login_default_client().await;

    if !fs::exists(".dummy").unwrap() {
        fs::create_dir(".dummy").expect("Dir already exists");
    }

    let result = fs::write(
        ".dummy/dummy.txt",
        "This is a dummy file. You are a dummy for downloading this file.",
    );
    if result.is_err() {
        panic!("Failed to write the temporary file we need to test this!");
    }

    let mut builder = TorrentCreatorBuilder::default();
    builder.source_path(PathBuf::from(".dummy"));
    builder.format(qbit::models::TorrentFormat::Hybrid);
    let torrent = builder.build().unwrap();

    let result = client
        .create_torrent(&torrent)
        .await
        .expect("Failed to upload our own custom torrent: ");
}
