use crate::{create_dummy_torrent, login_default_client};

/// This test ensures that the API correctly deserialize the torrents the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_basic_torrent() {
    let client = login_default_client().await;

    let result = create_dummy_torrent(&client).await;

    assert!(result.is_ok());
}

/// This test ensures that the API correctly deserialize the torrents the response.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_list_tasks() {
    let client = login_default_client().await;
    let result = create_dummy_torrent(&client).await.unwrap();
    let list = client.torrent_creator_status().await.unwrap();
    assert!(list.len() > 1);
    assert!(list.iter().any(|t| t.task_id == result.task_id));
}
