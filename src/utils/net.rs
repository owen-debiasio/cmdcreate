use reqwest::blocking::Client;
use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("cmdcreate-upgrader")
        .build()
        .expect("Failed to build HTTP client")
}

pub fn connected_to_internet() -> bool {
    match "1.1.1.1:53".to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                return TcpStream::connect_timeout(&addr, Duration::from_secs(3)).is_ok();
            }
            false
        }
        Err(_) => false,
    }
}
