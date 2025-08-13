use crate::{create_test_data, login_default_client};

#[tokio::test]
#[ignore = "Test hits api endpoint"]
pub async fn list_directory() {
    let folder = create_test_data();
    let client = login_default_client().await;
    let contents = client.get_directory_contents(&folder).await.unwrap();
    println!("{:?}", contents);

    assert!(contents.contains(&format!("{folder}/dummy.txt")));
}
