use dotenv::dotenv;
use qbit::{
    Api,
    models::{TorrentCreatorBuilder, TorrentCreatorTask, TorrentPieceSize},
    parameters::AddTorrentBuilder,
};
use std::{env, fs};

pub mod authentication;
pub mod sync;
pub mod torrents;

pub const DEBIAN_HASH: &str = "6f4370df4304609a8793ce2b59178dcc8febf5e2";
pub const DEBIAN_TRACKER: &str = "magnet:?xt=urn:btih:6f4370df4304609a8793ce2b59178dcc8febf5e2&dn=debian-12.11.0-amd64-netinst.iso&xl=702545920&tr=http%3A%2F%2Fbttracker.debian.org%3A6969%2Fannounce&ws=https://cdimage.debian.org/cdimage/archive/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso&ws=https://cdimage.debian.org/cdimage/release/12.11.0/amd64/iso-cd/debian-12.11.0-amd64-netinst.iso";

pub fn get_server_details() -> String {
    dotenv().ok();

    let url = env::var("url");
    let port = env::var("port");

    if url.is_err() || port.is_err() {
        println!("Default to `http://localhost:45378` as couldn't fully load data from .env");
        return String::from("http://localhost:45378");
    }

    let finished_url = format!("{}:{}", url.unwrap(), port.unwrap());
    println!("Using {} from .env file", finished_url);
    finished_url
}

pub fn get_server_username() -> String {
    env::var("username").unwrap_or("admin".to_string())
}

pub fn get_server_password() -> String {
    env::var("password").unwrap_or("adminadmin".to_string())
}

pub async fn login_default_client() -> Api {
    Api::new_login_username_password(
        &get_server_details(),
        &get_server_username(),
        &get_server_password(),
    )
    .await
    .expect("Failed to log in to the default client. Please check the server details, username, and password.")
}

pub async fn add_debian_torrent(client: &Api) {
    let param = AddTorrentBuilder::default()
        .torrents(vec![DEBIAN_TRACKER.to_string()])
        .paused(true)
        .build()
        .expect("Failed to build AddTorrent");

    client
        .add_torrent(param)
        .await
        .expect("Failed to add torrent");
    // Note: Added the stop call since the paused parameter doesn't work for some reason.
    client
        .stop(vec![DEBIAN_HASH])
        .await
        .expect("Failed to stop torrent");
}

pub async fn create_dummy_torrent(client: &Api) -> Result<TorrentCreatorTask, qbit::Error> {
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

    client.create_torrent(&torrent).await
}
