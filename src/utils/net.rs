use reqwest::blocking::Client;
use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::utils::sys::args_contains;

pub fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("cmdcreate-upgrader")
        .build()
        .expect("Failed to build HTTP client")
}

pub fn is_offline() -> bool {
    if args_contains("-o") || args_contains("--offline") {
        return true;
    }

    match "1.1.1.1:53".to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                return TcpStream::connect_timeout(&addr, Duration::from_secs(0)).is_err();
            }
            true
        }
        Err(_) => true,
    }
}
