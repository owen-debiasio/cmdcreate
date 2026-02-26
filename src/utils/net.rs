// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Owen Debiasio
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
                return TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_err();
            }
            true
        }
        Err(_) => true,
    }
}
