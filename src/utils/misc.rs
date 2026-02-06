use reqwest::blocking::Client;
use std::time::Duration;

pub fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("cmdcreate-upgrader")
        .build()
        .expect("Failed to build HTTP client")
}
