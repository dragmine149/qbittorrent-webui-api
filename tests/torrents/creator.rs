use crate::login_default_client;
use qbit::models::{TorrentCreatorBuilder, TorrentPieceSize};
use std::fs;

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
    builder.source_path(".dummy");
    builder.piece_size(TorrentPieceSize::m256());
    builder.start_seeding(true);
    let torrent = builder.build().unwrap();

    let result = client
        .create_torrent(&torrent)
        .await
        .expect("Failed to upload our own custom torrent: ");
    println!("{:?}", result);

    panic!("Panic!");
}
