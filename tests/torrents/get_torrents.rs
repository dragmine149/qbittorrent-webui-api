use crate::{DEBIAN_HASH, add_debian_torrent, login_deafult_client};

/// This test ensures that the API correctly deserialize the torrents the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn corectly_deserialize_torrents_from_respose() {
    let client = login_deafult_client().await;
    add_debian_torrent(&client).await;

    let torrents = client
        .torrents(None)
        .await
        .expect("Failed to fetch main data: ");

    assert!(!torrents.is_empty());
    assert!(torrents.into_iter().any(|x| x.hash == DEBIAN_HASH));
}
