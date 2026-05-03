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
    meta::{author_information::AUTHOR, project_information::PROJECT},
    output,
    utils::{
        colors::COLORS,
        io::{ask_for_confirmation, error},
        net::not_connected_to_internet,
        fs::{core::{create_folder, path_exists}, paths::PATHS, misc::{download_file_to_location_via_curl, use_pager_on_file}}
    },
};

use std::process::exit;

pub const YEAR: &str = "2026";

pub mod author_information {
    pub struct Author {
        pub name: &'static str,
        pub username: &'static str,
        pub email: &'static str,
    }

    pub const AUTHOR: Author = Author {
        name: "Owen Debiasio",
        username: "owen-debiasio",
        email: "owen.debiasio@gmail.com",
    };
}

pub mod project_information {
    pub struct Project {
        pub name: &'static str,
        pub repository: &'static str,
        pub repository_raw: &'static str,
    }

    pub const PROJECT: Project = Project {
        name: "cmdcreate",
        repository: "https://github.com/owen-debiasio/cmdcreate",
        repository_raw: "https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/",
    };
}

pub fn get_project_copyright_info() -> String {
    let project_author = AUTHOR.name;
    let author_email = AUTHOR.email;

    format!("Copyright {YEAR} {project_author} <{author_email}>")
}

pub fn display_full_license() {
    let (red, reset) = (COLORS.red, COLORS.reset);

    let path_to_license_file = &PATHS.license;

    if path_exists(path_to_license_file) {
        use_pager_on_file(path_to_license_file);
    } else {
        let error_message: &str = "License file not found!";

        let question = &format!(
            "{red}{error_message}{reset}{}",
            if not_connected_to_internet() {
                // Exit here now because there is no need to download and install the license
                // if user has no internet
                error(error_message, "")
            } else {
                " Do you want to download and install the license?"
            }
        );

        ask_for_confirmation(question, true);

        download_and_install_license();
    }

    exit(0)
}

fn download_and_install_license() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let raw_repo_path = PROJECT.repository_raw;
    let license_path = &PATHS.license;

    let license_install_directory = license_path.replace("LICENSE", "");
    create_folder(&license_install_directory);

    let license_download_path = &format!("{raw_repo_path}LICENSE");
    download_file_to_location_via_curl(license_path, license_download_path);

    license_install_success_check(license_path);

    output!("\n{green}Successfully downloaded license!{reset}");

    exit(0)
}

fn license_install_success_check(license_path: &str) {
    log(
        "meta::license_install_success_check(): \
        Determining License installation status...",
        Severity::Normal,
    );

    if !path_exists(license_path) {
        error("Failed to install license!", "License file not found!")
    }

    log(
        "meta::license_install_success_check(): \
        License installed correctly, continuing...",
        Severity::Normal,
    );
}
