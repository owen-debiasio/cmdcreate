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
    run_shell_command,
    utils::{
        fs::core::path_exists,
        io::error,
        net::not_connected_to_internet,
        sys::{command::system_command_is_installed, env::root_check},
    },
};

pub fn use_pager_on_file(file_path: &str) {
    if !system_command_is_installed("less") {
        error(
            "Failed to find pager (less).",
            Some("Executable not found."),
        )
    }

    if !path_exists(file_path) {
        error("Failed to page file!", Some("File not found!"))
    }

    run_shell_command!("less {file_path}");
}

pub fn install_binary(mode: &str, binary: &str, destination: &str) {
    root_check();

    log(
        &format!(
            "
            utils/fs::misc::install_binary(): \
            Installing binary \"{binary}\" \
            to \"{destination}\" \
            using mode \"{mode}\"..."
        ),
        Severity::Normal,
    );

    run_shell_command!("install {mode} {binary} {destination}");

    if !path_exists(destination) {
        error("Failed to install binary!", Some("Binary not found!"))
    }

    log(
        "utils/fs::misc::install_binary(): Successfully installed binary!",
        Severity::Normal,
    );
}

pub fn download_file_to_location_via_curl(
    file_destination: &str,
    path_of_file_to_be_downloaded: &str,
) {
    if not_connected_to_internet() {
        error(
            "Unable to retrieve file!",
            Some("Not connected to the internet!"),
        )
    }

    run_shell_command!(
        "
        curl -sSLo {file_destination} \
        {path_of_file_to_be_downloaded}"
    );

    if !path_exists(file_destination) {
        error("Downloaded file not found!", None)
    }
}
