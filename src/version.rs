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
        fs::{MAIN_PATH, path_exists, read_file_to_string, write_to_file},
        io::error,
        net::{http_client, not_connected_to_internet},
    },
};
use serde_json::Value;
use std::{cmp::Ordering, error::Error};

pub const CURRENT_PROJECT_VERSION: &str = "v1.2.8";

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

fn cache_build_information(path_of_cache: &str, information_to_cache: &str) {
    log(
        &format!(
            "version::cache_build_information(): \
            Caching information: \"{information_to_cache}\" \
            to file: \"{path_of_cache}\"..."
        ),
        Severity::Normal,
    );

    write_to_file(path_of_cache, information_to_cache, false)
        .expect("Failed to write cached information");

    if !path_exists(path_of_cache) {
        error("Failed to cache information", "File missing")
    }

    let cached_file_contents = read_file_to_string(path_of_cache);

    if !cached_file_contents.contains(information_to_cache) {
        error(
            "Failed to cache information",
            "File doesn't contain information",
        )
    }
}

pub fn get_latest_tag() -> String {
    fn get_tag_from_repository() -> String {
        let repository_owner = AUTHOR.username;
        let repository = PROJECT.name;

        if not_connected_to_internet() {
            log(
                "version::get_latest_tag::get_tag_from_repository(): \
                No internet... Unable to retrieve latest tag...",
                Severity::Warn,
            );

            return "unknown".to_string();
        }

        let retrieved_api_contents: Result<String, Box<dyn Error>> = (|| {
            let tag_api_retrieval_url = format!(
                "https://api.github.com/repos/{repository_owner}/{repository}/releases/latest"
            );

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

        match retrieved_api_contents {
            Ok(latest_tag) => {
                log(
                    &format!(
                        "version::get_latest_tag::get_tag_from_repository(): Latest tag: {latest_tag}"
                    ),
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

    log(
        "version::get_latest_tag(): Retrieving latest tag...",
        Severity::Normal,
    );

    let file_to_store_tag = &format!("{MAIN_PATH}/LATEST_TAG");

    // Use existing cached tag

    if path_exists(file_to_store_tag) {
        log(
            "version::get_latest_tag(): Cached tag exists, using...",
            Severity::Normal,
        );

        let extracted_tag = read_file_to_string(file_to_store_tag);

        return extracted_tag;
    }

    log(
        "version::get_latest_tag(): Latest tag not cached, retrieving...",
        Severity::Normal,
    );

    let latest_tag = get_tag_from_repository();

    cache_build_information(file_to_store_tag, &latest_tag);

    latest_tag
}

pub fn get_latest_commit() -> String {
    fn get_latest_commit_from_repository() -> String {
        let repo_owner = AUTHOR.username;
        let repository = PROJECT.name;

        let commit_hash_url =
            format!("https://api.github.com/repos/{repo_owner}/{repository}/commits/main");

        let extracted_commit_from_hash: Value = http_client()
            .get(commit_hash_url)
            .send()
            .expect("request failed")
            .json()
            .expect("invalid json");

        extracted_commit_from_hash["sha"]
            .as_str()
            .expect("missing sha")
            .chars()
            .take(7)
            .collect::<String>()
    }

    log(
        "version::get_latest_commit(): Retrieving latest commit...",
        Severity::Normal,
    );

    let file_to_store_commit = &format!("{MAIN_PATH}/LATEST_COMMIT");

    log(
        "version::get_latest_commit(): Latest commit not cached, retrieving...",
        Severity::Normal,
    );

    // Use existing cached commit

    if path_exists(file_to_store_commit) {
        log(
            "version::get_latest_commit(): Cached commit exists, using...",
            Severity::Normal,
        );

        let extracted_tag = read_file_to_string(file_to_store_commit);

        return extracted_tag;
    }

    // Cache the tag

    let latest_commit = get_latest_commit_from_repository();

    cache_build_information(file_to_store_commit, &latest_commit);

    latest_commit
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
