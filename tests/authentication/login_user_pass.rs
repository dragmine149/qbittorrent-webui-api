use qbit::Api;

#[tokio::test]
async fn login_url_1() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090/", "admin", "adminadmin").await;

    if result.is_err() {
        eprintln!("{:?}", result.as_ref().err().unwrap());
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn login_url_2() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090", "admin", "adminadmin").await;

    if result.is_err() {
        eprintln!("{:?}", result.as_ref().err().unwrap());
    }

    assert!(result.is_ok());
}

#[tokio::test]
async fn incorrect_username() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090/", "fjiooiaaso", "adminadmin")
            .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn incorrect_password() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090/", "admin", "snkabjhioahsio").await;

    assert!(result.is_err());
}
