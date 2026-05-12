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
    commands::update::update,
    logger::{Severity, log},
    meta::{author_information::AUTHOR, get_project_copyright_info, project_information::PROJECT},
    output,
    utils::{
        colors::COLORS,
        io::error,
        net::{not_connected_to_internet, ureq_agent},
    },
};
use serde_json::{Value, from_reader};
use std::{cmp::Ordering, error::Error};

pub const CURRENT_PROJECT_VERSION: &str = "v1.3.4";

pub fn version_is_development_build() -> bool {
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

    let latest_retrieved_tag = &get_latest_tag();
    let version_to_parse = &parse_version(latest_retrieved_tag);

    match parse_version(CURRENT_PROJECT_VERSION).cmp(version_to_parse) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_build_status() -> &'static str {
    if version_is_development_build() {
        if not_connected_to_internet() {
            "(build status unknown)"
        } else {
            "(development)"
        }
    } else {
        "(stable)"
    }
}

fn fetch_github_json(endpoint: &str) -> Result<Value, Box<dyn Error>> {
    let repo_api_url = PROJECT.repository_api.trim_end_matches('/');
    let github_json_url = format!("{repo_api_url}/{endpoint}");

    let api_response = ureq_agent()
        .get(&github_json_url)
        .header("User-Agent", PROJECT.name)
        .call()
        .expect("Failed to call user agent")
        .into_body()
        .into_reader();

    let retrieved_json = from_reader(api_response)?;

    Ok(retrieved_json)
}

pub fn get_latest_tag() -> String {
    if not_connected_to_internet() {
        log("version::get_latest_tag(): No internet...", Severity::Warn);
        return "unknown".to_string();
    }

    match fetch_github_json("releases/latest") {
        Ok(json) => {
            let tag = json["tag_name"].as_str().unwrap_or("unknown").to_string();
            log(
                &format!("version::get_latest_tag(): Latest tag: {tag}"),
                Severity::Normal,
            );
            tag
        }
        Err(tag_retrieval_error) => {
            log(
                &format!("version::get_latest_tag(): Error: {tag_retrieval_error}"),
                Severity::Warn,
            );
            "unknown".to_string()
        }
    }
}

pub fn get_latest_commit() -> String {
    if not_connected_to_internet() {
        return "unknown".to_string();
    }

    match fetch_github_json("commits/main") {
        Ok(json) => {
            let full_sha = json["sha"].as_str().unwrap_or("");
            let short_sha = full_sha.chars().take(7).collect::<String>();

            if short_sha.is_empty() {
                return "unknown".to_string();
            }

            log(
                &format!("version::get_latest_commit(): Short SHA: {short_sha}"),
                Severity::Normal,
            );
            short_sha
        }
        Err(commit_retrieval_error) => {
            log(
                &format!("version::get_latest_commit(): Error: {commit_retrieval_error}"),
                Severity::Warn,
            );
            "unknown".to_string()
        }
    }
}

pub fn print_version_info() {
    let project_name = PROJECT.name;

    let project_author = AUTHOR.name;
    let project_author_email = AUTHOR.email;

    let project_copyright_info = get_project_copyright_info();

    let build_status = get_build_status();

    output!(
        "
{project_name} {CURRENT_PROJECT_VERSION} {build_status}
Copyright (C) {project_copyright_info}.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by {project_author} <{project_author_email}>.
        ",
        true
    );
}

pub fn check() {
    let (blue, green, magenta) = (COLORS.blue, COLORS.green, COLORS.magenta);

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            None,
        )
    }

    output!("\nChecking for updates...", true);

    let latest_stable_version = get_latest_tag();

    if version_is_development_build() {
        output!(
            "\nYou are running a newer version {magenta}({CURRENT_PROJECT_VERSION}){blue} \
            than the latest release {green}({latest_stable_version}).",
            true
        );

        return;
    }

    if CURRENT_PROJECT_VERSION != latest_stable_version {
        output!(
            "{green}\nUpdate available: {CURRENT_PROJECT_VERSION} -> {latest_stable_version}\n",
            true
        );

        update();

        return;
    }

    output!("Already up to date.", true);
}
