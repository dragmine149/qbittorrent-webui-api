use dotenv::dotenv;
use qbit::Api;
use std::env;

fn get_server_details() -> String {
    dotenv().ok();

    let url = env::var("url");
    let port = env::var("port");

    if url.is_err() || port.is_err() {
        println!("Default to `http://localhost:45378` as couldn't fully load data from .env");
        return String::from("http://localhost:45378");
    }

    let finished_url = format!("{}:{}", url.unwrap(), port.unwrap());
    println!("Using {} from .env file", finished_url);
    finished_url
}

fn get_server_username() -> String {
    println!("{}", env::var("username").unwrap_or("admin".to_string()));
    env::var("username").unwrap_or("admin".to_string())
}

fn get_server_password() -> String {
    println!(
        "{}",
        env::var("password").unwrap_or("adminadmin".to_string())
    );
    env::var("password").unwrap_or("adminadmin".to_string())
}

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
