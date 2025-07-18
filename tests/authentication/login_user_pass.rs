use qbit::Api;

#[tokio::test]
async fn incorrect_username() {
    let result =
        Api::new_login_username_password("http://localhost:45378/", "fjiooiaaso", "adminadmin")
            .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn incorrect_password() {
    let result =
        Api::new_login_username_password("http://localhost:45378/", "admin", "snkabjhioahsio")
            .await;

    assert!(result.is_err());
}
