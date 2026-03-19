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

use std::process::exit;

use crate::utils::{
    colors::COLORS,
    fs::{PATHS, create_folder, path_exists, read_file_to_string},
    io::{ask_for_confirmation, error},
    net::not_connected_to_internet,
    sys::run_shell_command,
};

pub const YEAR: &str = "2026";

pub const AUTHOR: &str = "Owen Debiasio";
pub const AUTHOR_USERNAME: &str = "owen-debiasio";
pub const AUTHOR_EMAIL: &str = "owen.debiasio@gmail.com";

pub const PROJECT_NAME: &str = "cmdcreate";

pub fn get_project_copyright_info() -> String {
    // So fucking annoyed that this can't be a static
    format!("Copyright {YEAR} {AUTHOR} <{AUTHOR_EMAIL}>")
}

pub fn display_full_license() {
    let (red, reset) = (COLORS.red, COLORS.reset);

    let path_to_license_file = &PATHS.license;

    if path_exists(path_to_license_file) {
        let license_file_contents = read_file_to_string(path_to_license_file);

        println!("{license_file_contents}");
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
}

fn download_and_install_license() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let license_path = &PATHS.license;
    let license_install_directory = license_path.replace("LICENSE", "");

    create_folder(&license_install_directory).expect("Failed to create folder");

    let command_to_download_and_install_license: &str = &format!(
        "curl -sSLo \
        {license_path} \
        https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/LICENSE"
    );

    run_shell_command(command_to_download_and_install_license);

    println!("\n{green}Successfully downloaded license!{reset}");

    exit(0)
}
