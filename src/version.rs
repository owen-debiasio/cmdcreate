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

use crate::{
    logger::log,
    meta::{AUTHOR, AUTHOR_EMAIL, AUTHOR_USERNAME, PROJECT_NAME, get_project_copyright_info},
    utils::{
        io::error,
        net::{http_client, not_connected_to_internet},
        sys::run_shell_command,
    },
};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const CURRENT_PROJECT_VERSION: &str = "v1.1.9";

pub fn version_is_development_build() -> bool {
    // TODO: Make this a standalone function
    let parse_version = |parsed_version_digits: &str| -> (u32, u32, u32) {
        // 'v' always comes before the version
        // '.' separates the version values

        let version_digits: Vec<u32> = parsed_version_digits
            .trim_start_matches('v')
            .split('.')
            .map(|version_digit_splitter| version_digit_splitter.parse().unwrap_or(0))
            .collect();
        (
            *version_digits.first().unwrap_or(&0),
            *version_digits.get(1).unwrap_or(&0),
            *version_digits.get(2).unwrap_or(&0),
        )
    };

    match parse_version(CURRENT_PROJECT_VERSION).cmp(&parse_version(&get_latest_tag_from_repo(
        AUTHOR_USERNAME,
        PROJECT_NAME,
    ))) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_latest_tag_from_repo(owner: &str, repo: &str) -> String {
    if not_connected_to_internet() {
        log(
            "version::get_latest_tag(): No internet... Unable to retrieve latest tag...",
            1,
        );
        return "unknown".to_string();
    }

    let result: Result<String, Box<dyn Error>> = (|| {
        let tag_api_retrieval_url =
            format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");

        let api_response = http_client()
            .get(tag_api_retrieval_url)
            .header("User-Agent", "rust-app")
            .send()?;

        let api_response_as_json: Value = api_response.json()?;

        let tag_retrieved_via_api = api_response_as_json["tag_name"]
            .as_str()
            .ok_or("Missing tag_name")?
            .to_owned();

        Ok(tag_retrieved_via_api)
    })();

    match result {
        Ok(latest_tag) => {
            log(
                &format!("version::get_latest_tag(): Latest tag: {latest_tag}"),
                0,
            );
            latest_tag
        }
        Err(tag_retrieve_error) => error(
            "Unable to retrieve latest tag:",
            &tag_retrieve_error.to_string(),
        ),
    }
}

pub fn get_latest_commit_from_repo(owner: &str, repo: &str, branch: &str) -> String {
    let commit_hash_url = format!("https://api.github.com/repos/{owner}/{repo}/commits/{branch}");

    let extracted_commit_from_hash: Value = http_client()
        .get(commit_hash_url)
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    let commit = extracted_commit_from_hash["sha"]
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

pub fn print_version_info() {
    println!(
        "
cmdcreate {CURRENT_PROJECT_VERSION}
Copyright (C) {}.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by {AUTHOR} <{AUTHOR_EMAIL}>.
        ",
        get_project_copyright_info()
    );
}

pub fn print_version_changelog() {
    if not_connected_to_internet() {
        error("You need internet to retrieve the changelog.", "")
    }

    log("main::main(): Displaying changelog...", 0);

    run_shell_command(
        "curl -L https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/changes.md",
    );
}
