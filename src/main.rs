use qbit::{
    Api,
    models::ContentLayout,
    parameters::{AddTorrent, AddTorrentType},
};

#[tokio::main]
async fn main() {
    let api = Api::new_login_username_password("http://127.0.0.1:8090", "dragon", "torrent")
        .await
        .unwrap();

    println!("{:#?}", api.preferences().await.unwrap());

    // let mut torrent = AddTorrent::new();
    // torrent.torrents = AddTorrentType::Links(vec![
    //     "magnet:?xt=urn:btih:d882d6f8ac68c173fb86a6fc65249f0a7c749f47&dn=%5BJudas%5D%20Chuunibyou%20demo%20Koi%20ga%20Shitai%21%20%28Love%2C%20Chunibyo%20%26%20Other%20Delusions%29%20%28Seasons%201-2%20%2B%20Movies%201-2%29%20%5BBD%201080p%5D%5BHEVC%20x265%2010bit%5D%5BDual-Audio%5D%5BEng-Subs%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce".to_string(),
    // ]);
    // torrent.savepath = Some("/home/dragmine/Downloads/anime/1".to_string());
    // torrent.content_layout = ContentLayout::NoSubFolder;

    // let e = api.add_torrent(torrent).await;
    // e.unwrap()
}
