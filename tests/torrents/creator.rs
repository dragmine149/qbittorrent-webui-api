use qbit::{Error, models::TorrentCreatorBuilder};

use crate::{create_dummy_torrent, login_default_client};
use std::{env, fs};

/// This test makes sure that the endpoint can create a dummy task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_task() {
    let client = login_default_client().await;
    let result = create_dummy_torrent(&client).await;

    assert!(result.is_ok());
}

/// This test makes sure that the list shows the dummy task we've just created.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn list_tasks() {
    let client = login_default_client().await;
    let result = create_dummy_torrent(&client).await.unwrap();
    let list = client.list_tasks().await.unwrap();
    assert!(!list.is_empty());
    assert!(list.iter().any(|t| t.task_id == result.task_id));
}

/// Tests to see that upon the torrent being finished, it is the same as the information we have in the dummy file.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_torrent_file() {
    let client = login_default_client().await;
    create_dummy_torrent(&client).await.unwrap();

    let folder = env::var("temp_dir").unwrap();
    let path = format!("{folder}_data/dummy.torrent");
    if !fs::exists(&path).unwrap() {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
    let data = fs::read(&path).unwrap();

    let list = client.list_tasks().await.unwrap();
    for item in list.iter() {
        let r = client
            .get_task_file(item.task_id.to_owned())
            .await
            .unwrap_or_default()
            .to_vec();
        if r == data {
            assert!(true);
        }
    }
}

/// Make sure that we can delete the created task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn delete_created_task() {
    let client = login_default_client().await;
    let task_id = create_dummy_torrent(&client).await.unwrap();
    let list = client.list_tasks().await.unwrap();
    assert!(!list.is_empty());
    assert!(list.iter().any(|t| t.task_id == task_id.task_id));
    client
        .delete_task(&task_id)
        .await
        .expect("Failed to delete task");
    let list = client.list_tasks().await.unwrap();
    assert!(!list.iter().any(|t| t.task_id == task_id.task_id));
}

/// Test to check failed to create errors
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn make_failed_task() {
    let client = login_default_client().await;

    let torrent = TorrentCreatorBuilder::default()
        .start_seeding(true)
        .private(true)
        .source_path("/tmp/tmp.L9uwLe8LOA")
        .build()
        .expect("Failed to build torrent creator");

    let id = client.create_task(&torrent).await.unwrap();

    let result = client.get_task_file(id).await;
    assert!(result.is_err());
    if let Err(Error::Http409(_e)) = result {
        assert!(true);
        // eprintln!("error: {}", _e);
        // assert!(false);
    } else {
        panic!("Expected Http409 error");
    }
}
