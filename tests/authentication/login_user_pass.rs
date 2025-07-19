use crate::{get_server_details, get_server_password, get_server_username};
use qbit::Api;

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn correct_credentials() {
    let result = Api::new_login_username_password(
        &get_server_details(),
        &get_server_username(),
        &get_server_password(),
    )
    .await;

    if result.is_err() {
        println!("Err: {:?}", result.err().unwrap());
        assert!(false);
    }

    assert!(true);
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn incorrect_username() {
    let result = Api::new_login_username_password(
        &get_server_details(),
        "fjiooiaaso",
        &get_server_password(),
    )
    .await;

    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), qbit::Error::AuthFailed(_)));
}

#[tokio::test]
#[ignore = "Test hits api endpoint"]
async fn incorrect_password() {
    let result = Api::new_login_username_password(
        &get_server_details(),
        &get_server_username(),
        "snkabjhioahsio",
    )
    .await;

    assert!(result.is_err());
    assert!(matches!(result.err().unwrap(), qbit::Error::AuthFailed(_)));
}
