use crate::{create_dummy_torrent, login_default_client};
use qbit::models::TaskStatus;
use std::{env, fs};

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
    let list = client.task_list().await.unwrap();
    assert!(list.len() >= 1);
    assert!(list.iter().any(|t| t.task_id == result.task_id));
}

/// Tests to see that upon the torrent being finished, it is the same as the information we have in the dummy file.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_torrent_file() {
    let client = login_default_client().await;
    create_dummy_torrent(&client).await.unwrap();

    // This... might not be the best way to go around this, but it works (somehow).
    let mut task_id = String::new();
    while task_id.is_empty() {
        let list = client.task_list().await.unwrap();
        if let Some(stat) = list
            .iter()
            .filter(|v| v.status == TaskStatus::Finished)
            .next()
        {
            task_id = stat.task_id.clone();
        }
    }

    let r = client.get_task_file(task_id).await.unwrap();

    let folder = env::var("temp_dir").unwrap();
    assert_eq!(fs::read(format!("{folder}_data/dummy.torrent")).unwrap(), r);
}

/// Make sure that we can delete the created task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn delete_created_task() {
    let client = login_default_client().await;
    let task_id = create_dummy_torrent(&client).await.unwrap();
    let list = client.task_list().await.unwrap();
    assert!(list.len() >= 1);
    assert!(list.iter().any(|t| t.task_id == task_id.task_id));
    client
        .delete_task(&task_id)
        .await
        .expect("Failed to delete task");
    let list = client.task_list().await.unwrap();
    assert!(!list.iter().any(|t| t.task_id == task_id.task_id));
}
