use dotenv::dotenv;
use std::env;

pub mod authentication;

pub fn get_server_details() -> String {
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

pub fn get_server_username() -> String {
    env::var("username").unwrap_or("admin".to_string())
}

pub fn get_server_password() -> String {
    env::var("password").unwrap_or("adminadmin".to_string())
}
