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
    core::{
        logger::{consts::Severity, main::log},
        meta::project::project_information::PROJECT,
    },
    output, run_shell_command,
    utils::{
        colors::COLORS,
        fs::core::path_exists,
        io::{error, output_is_silent},
        net::{not_connected_to_internet, ureq_agent},
        sys::command::system_command_is_installed,
    },
};

use serde_json::{Value, from_reader};
use std::error::Error;

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
        log(
            "utils/git::get_latest_tag(): No internet...",
            Severity::Warn,
        );
        return "unknown".to_string();
    }

    match fetch_github_json("releases/latest") {
        Ok(json) => {
            let tag = json["tag_name"].as_str().unwrap_or("unknown").to_string();
            log(
                &format!("utils/git::get_latest_tag(): Latest tag: {tag}"),
                Severity::Normal,
            );
            tag
        }
        Err(tag_retrieval_error) => {
            log(
                &format!("utils/git::get_latest_tag(): Error: {tag_retrieval_error}"),
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
                &format!("utils/git::get_latest_commit(): Short SHA: {short_sha}"),
                Severity::Normal,
            );
            short_sha
        }
        Err(commit_retrieval_error) => {
            log(
                &format!("utils/git::get_latest_commit(): Error: {commit_retrieval_error}"),
                Severity::Warn,
            );
            "unknown".to_string()
        }
    }
}

pub fn clone_repository(destination: &str) {
    log(
        "utils/git::clone_repository(): Cloning project repository...",
        Severity::Normal,
    );

    if !system_command_is_installed("git") {
        error(
            "Unable to clone repository.",
            Some("Executable \"git\" not installed."),
        )
    }

    output!("\nCloning project repository...", true);

    let project_repo = PROJECT.repository;

    let clone_silently = if output_is_silent() { "--quiet" } else { "" };

    run_shell_command!(
        "git clone {clone_silently} --depth=1 \
        {project_repo}.git {destination}"
    );

    if !path_exists(destination) {
        error(
            "Failed to clone repository!",
            Some("Destination not found!"),
        )
    }

    log(
        &format!(
            "utils/git::clone_repository(): \
            Successfully cloned repository \"{project_repo}\""
        ),
        Severity::Normal,
    );
}
