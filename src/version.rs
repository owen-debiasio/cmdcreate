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
    logger::{Severity, log},
    meta::{author_information::AUTHOR, get_project_copyright_info, project_information::PROJECT},
    output,
    utils::{
        colors::COLORS,
        fs::{delete_file, download_file_to_location_via_curl, read_file_to_string},
        io::error,
        net::{http_client, not_connected_to_internet},
    },
};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const CURRENT_PROJECT_VERSION: &str = "v1.2.6";

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

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_retrieved_tag = &get_latest_tag_from_repo(author_username, project_name);
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

pub fn get_latest_tag_from_repo(owner: &str, repo: &str) -> String {
    if not_connected_to_internet() {
        log(
            "version::get_latest_tag_from_repo(): No internet... Unable to retrieve latest tag...",
            Severity::Warn,
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
                &format!("version::get_latest_tag_from_repo(): Latest tag: {latest_tag}"),
                Severity::Normal,
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
        &format!(
            "
            version::get_latest_commit_from_repo(): \
            Retrieved latest commit: \"{commit}\""
        ),
        Severity::Normal,
    );

    commit
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

pub fn print_version_changelog() {
    if not_connected_to_internet() {
        error("You need internet to retrieve the changelog.", "")
    }

    log(
        "version::print_version_changelog(): Displaying changelog...",
        Severity::Normal,
    );

    let repo_path = PROJECT.repository_raw;
    let changelog_download_path = &format!("{repo_path}/changes.md");

    let changelog_path = "/tmp/cmdcreate-changelog.md";
    download_file_to_location_via_curl(changelog_path, changelog_download_path);

    let changelog_file_contents = read_file_to_string(changelog_path);
    output!("{changelog_file_contents}", false);

    delete_file(changelog_path).expect("Failed to delete changelog");
}
