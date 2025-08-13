use crate::{create_dummy_torrent, login_default_client};

/// This test makes sure that the endpoint can create a dummy task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_basic_torrent() {
    let client = login_default_client().await;

    let result = create_dummy_torrent(&client).await;

    assert!(result.is_ok());
}

/// This test makes sure that the list shows the dummy task we've just created.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_list_tasks() {
    let client = login_default_client().await;
    let result = create_dummy_torrent(&client).await.unwrap();
    let list = client.torrent_creator_status().await.unwrap();
    assert!(list.len() >= 1);
    assert!(list.iter().any(|t| t.task_id == result.task_id));
}

/// Make sure that we can delete the created task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn delete_created_task() {
    let client = login_default_client().await;
    let task_id = create_dummy_torrent(&client).await.unwrap();
    let list = client.torrent_creator_status().await.unwrap();
    assert!(list.len() >= 1);
    assert!(list.iter().any(|t| t.task_id == task_id.task_id));
    client
        .delete_task(&task_id)
        .await
        .expect("Failed to delete task");
    let list = client.torrent_creator_status().await.unwrap();
    assert!(!list.iter().any(|t| t.task_id == task_id.task_id));
}
