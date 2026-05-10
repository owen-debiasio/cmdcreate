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

pub const CURRENT_PROJECT_VERSION: &str = "v1.3.3";

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

    let latest_retrieved_tag = &get_latest_tag_from_repo();
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

pub fn get_latest_tag_from_repo() -> String {
    if not_connected_to_internet() {
        log(
            "version::get_latest_tag_from_repo(): No internet...",
            Severity::Warn,
        );
        return "unknown".to_string();
    }

    let repository_api_url = format!("{}/releases/latest", PROJECT.repository_api);

    let result: Result<String, Box<dyn Error>> = (|| {
        let mut response = ureq_agent().get(&repository_api_url).call()?;

        let api_response_as_json: Value = from_reader(response.body_mut().as_reader())?;

        let tag = api_response_as_json["tag_name"]
            .as_str()
            .ok_or("Missing tag_name")?
            .to_owned();

        Ok(tag)
    })();

    match result {
        Ok(latest_tag) => {
            log(
                &format!("version::get_latest_tag_from_repo(): Latest tag: {latest_tag}"),
                Severity::Normal,
            );
            latest_tag
        }
        Err(tag_error) => error(
            "Unable to retrieve latest tag:",
            Some(&tag_error.to_string()),
        ),
    }
}

pub fn get_latest_commit_from_repo() -> String {
    let repository_api_url = format!("{}/commits/main", PROJECT.repository_api);

    let result: Result<String, Box<dyn Error>> = (|| {
        let mut api_response = ureq_agent().get(&repository_api_url).call()?;

        let extracted_json: Value = from_reader(api_response.body_mut().as_reader())?;

        let full_sha = extracted_json["sha"].as_str().ok_or("missing sha")?;
        Ok(full_sha.chars().take(7).collect())
    })();

    match result {
        Ok(commit_retrieved) => {
            log(
                &format!(
                    "version::get_latest_commit_from_repo(): Retrieved latest commit: \"{commit_retrieved}\""
                ),
                Severity::Normal,
            );
            commit_retrieved
        }
        Err(commit_retrieval_error) => {
            log(
                &format!(
                    "version::get_latest_commit_from_repo(): Failed to get commit: {commit_retrieval_error}"
                ),
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

    let latest_stable_version = get_latest_tag_from_repo();

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
