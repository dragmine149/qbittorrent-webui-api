use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// As the torrent is automatically stopped (in theory). We should confirm it's stopped.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn torrent_is_stopped() {
    let client = login_default_client().await;
    add_debian_torrent(&client).await;

    let torrents = client
        .torrents(None)
        .await
        .expect("Failed to fetch main data:");

    assert!(
        torrents
            .iter()
            .filter(|t| t.hash == DEBIAN_HASH)
            .next()
            .unwrap()
            .state
            .is_stopped()
    )
}
