use crate::{DEBIAN_HASH, add_debian_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the torrents field from the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correctly_deserialize_from_response() {
    let client = login_default_client(false).await;
    add_debian_torrent(&client).await;

    let res = client
        .main_data(None)
        .await
        .expect("Failed to fetch main data: ");

    assert!(res.torrents.is_some());
    let torrents = res.torrents.unwrap();
    assert!(!torrents.is_empty());
    assert!(torrents.contains_key(DEBIAN_HASH));
}
