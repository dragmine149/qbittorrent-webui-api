use crate::{create_dummy_torrent, login_default_client};
use qbit::parameters::{AddTorrentBuilder, AddTorrentType, TorrentFile};
use std::{env, fs};

/// This test makes sure that the endpoint can create a dummy task.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn create_task() {
    let client = login_default_client(false).await;
    let result = create_dummy_torrent(&client).await;

    assert!(result.is_ok());
}

/// This test makes sure that the list shows the dummy task we've just created.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn list_tasks() {
    let client = login_default_client(false).await;
    let result = create_dummy_torrent(&client).await.unwrap();
    let list = client.list_tasks().await.unwrap();
    assert!(!list.is_empty());
    assert!(list.iter().any(|t| t.task_id == result.task_id));
}

/// Tests to see that upon the torrent being finished, it is the same as the information we have in the dummy file.
#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn get_torrent_file() {
    let client = login_default_client(false).await;
    create_dummy_torrent(&client).await.unwrap();

    let folder = env::var("temp_dir").unwrap();
    let data = fs::read(format!("{folder}_data/dummy.torrent")).unwrap();

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
    let client = login_default_client(false).await;
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

/// Can another torrent download the created file
#[tokio::test]
#[ignore = "Test hits api endpoint (and test requires a second user!)"]
async fn download_created_torrent() {
    let client = login_default_client(false).await;
    create_dummy_torrent(&client)
        .await
        .expect("Failed to create dummy task");

    // no need to do dotenv as already done in previous function call.
    let folder = env::var("temp_dir").unwrap();

    fs::read_dir(format!("{folder}_data"))
        .unwrap()
        .for_each(|d| println!("{:?}", d));

    let file = TorrentFile {
        filename: String::from("dummy.torrent"),
        data: fs::read(format!("{folder}_data/dummy.torrent")).unwrap(),
    };

    let client2 = login_default_client(true).await;
    client2
        .add_torrent(
            AddTorrentBuilder::default()
                .torrents(AddTorrentType::from(file))
                .build()
                .unwrap(),
        )
        .await
        .expect("Failed to add torrent");

    assert!(
        client2
            .torrents(None)
            .await
            .unwrap()
            .iter()
            .find(|t| t.name == ".temp")
            .is_some()
    )
}
