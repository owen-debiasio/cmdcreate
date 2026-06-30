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
    output,
    utils::{
        colors::COLORS,
        fs::{
            core::creation::create_folder,
            misc::{download_file_to_location_via_curl, use_pager_on_file},
            paths::{PATHS, path_exists},
        },
        io::{ask_for_confirmation, error},
        net::not_connected_to_internet,
        sys::{distro::get_distro_base, env::running_as_root},
    },
};

use std::process::exit;

pub fn get_license_location() -> &'static str {
    if !running_as_root() {
        return "~/.local/share/doc/cmdcreate/LICENSE";
    }

    match get_distro_base() {
        "Debian" => "/usr/share/doc/cmdcreate/copyright/LICENSE",
        "Arch" => "/usr/share/licenses/cmdcreate/LICENSE",
        "Fedora" => "/usr/share/doc/cmdcreate/LICENSE",
        _ => "/usr/local/share/doc/cmdcreate/LICENSE",
    }
}

pub fn display_full() {
    let (red, reset) = (COLORS.red, COLORS.reset);

    let path_to_license_file = &PATHS.license;

    if path_exists(path_to_license_file) {
        use_pager_on_file(path_to_license_file);
    } else {
        let error_message = "License file not found!";

        let question = &format!(
            "{red}{error_message}{reset}{}",
            if not_connected_to_internet() {
                // Exit here now because there is no need to download and install the license
                // if user has no internet
                error(error_message, None)
            } else {
                " Do you want to download and install the license?"
            }
        );

        ask_for_confirmation(question, true);

        install();
    }

    exit(0)
}

fn install() {
    let green = COLORS.green;

    let raw_repo_path = PROJECT.repository_raw;
    let license_path = &PATHS.license;

    output!("Installing license to: {green}{license_path}");

    let license_install_directory = license_path.replace("LICENSE", "");
    create_folder(&license_install_directory);

    let license_download_path = &format!("{raw_repo_path}LICENSE");
    download_file_to_location_via_curl(license_path, license_download_path);

    install_success_check(license_path);

    output!("\n{green}Successfully downloaded license!");

    exit(0)
}

fn install_success_check(license_path: &str) {
    log(
        "core/meta/license::install_success_check(): \
        Determining License installation status...",
        Severity::Normal,
    );

    if !path_exists(license_path) {
        error(
            "Failed to install license!",
            Some("License file not found!"),
        )
    }

    log(
        "core/meta/license::install_success_check(): \
        License installed correctly, continuing...",
        Severity::Normal,
    );
}
