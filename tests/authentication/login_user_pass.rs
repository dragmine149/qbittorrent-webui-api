use qbit::{Api, Error};
use url::ParseError::RelativeUrlWithoutBase;

#[tokio::test]
async fn login_url_1() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090/", "admin", "torrent").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn login_url_2() {
    let result =
        Api::new_login_username_password("http://127.0.0.1:8090", "admin", "torrent").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn login_url_3() {
    let result = Api::new_login_username_password("127.0.0.1:8090", "admin", "torrent").await;

    assert!(result.is_err());
    let err = result.err().unwrap();

    assert!(matches!(err, Error::InvalidURL(RelativeUrlWithoutBase)));
}
