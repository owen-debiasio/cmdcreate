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
    let user_agent = "cmdcreate-updater";

    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent(user_agent)
        .build()
        .expect("Failed to build HTTP client")
}

fn internet_is_forced_disabled() -> bool {
    args_contains("-o") || args_contains("--offline")
}

pub fn not_connected_to_internet() -> bool {
    if internet_is_forced_disabled() {
        return true;
    }

    // The sample DNS is set to Cloudflare for reliability
    let sample_dns = "1.1.1.1:53";

    match sample_dns.to_socket_addrs() {
        Ok(mut socket_address) => {
            if let Some(addr) = socket_address.next() {
                return TcpStream::connect_timeout(&addr, Duration::from_secs(1)).is_err();
            }
            true
        }
        Err(_) => true,
    }
}
