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

use crate::meta::{AUTHOR_EMAIL, get_copyright};
use crate::{
    logger::log,
    meta::AUTHOR,
    utils::{
        io::error,
        net::{http_client, is_offline},
    },
};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const VERSION: &str = "v1.1.5";

pub fn is_development_version() -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
        let nums: Vec<u32> = v
            .trim_start_matches('v')
            .split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        (
            *nums.first().unwrap_or(&0),
            *nums.get(1).unwrap_or(&0),
            *nums.get(2).unwrap_or(&0),
        )
    };

    match parse_version(VERSION).cmp(&parse_version(&get_latest_tag(
        "owen-debiasio",
        "cmdcreate",
    ))) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_latest_tag(owner: &str, repo: &str) -> String {
    if is_offline() {
        log(
            "version::get_latest_tag(): No internet... Unable to retrieve latest tag...",
            1,
        );
        return "unknown".to_string();
    }

    let result: Result<String, Box<dyn Error>> = (|| {
        let response = http_client()
            .get(format!(
                "https://api.github.com/repos/{owner}/{repo}/releases/latest"
            ))
            .header("User-Agent", "rust-app")
            .send()?;

        let json: Value = response.json()?;

        Ok(json["tag_name"]
            .as_str()
            .ok_or("Missing tag_name")?
            .to_owned())
    })();

    match result {
        Ok(tag) => {
            log(&format!("version::get_latest_tag(): Latest tag: {tag}"), 0);
            tag
        }
        Err(e) => error("Unable to retrieve latest tag:", &e.to_string()),
    }
}

pub fn get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    let commit: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits/{branch}"
        ))
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    let commit = commit["sha"]
        .as_str()
        .expect("missing sha")
        .chars()
        .take(7)
        .collect::<String>();

    // And THIS is why cmdcreate can take forever to load on weak systems.
    log(
        &format!("version::get_latest_commit(): Retrieved latest commit: \"{commit}\""),
        0,
    );

    commit
}

pub fn print_info() {
    println!(
        "
cmdcreate {VERSION}
Copyright (C) {}.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by {AUTHOR} <{AUTHOR_EMAIL}>.
        ",
        get_copyright()
    );
}
