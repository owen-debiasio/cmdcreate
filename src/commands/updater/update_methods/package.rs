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
    core::meta::project::project_information::PROJECT,
    input, output, run_shell_command,
    utils::{
        colors::COLORS,
        fs::{
            core::delete_file,
            misc::{download_file_to_location_via_curl, install_binary},
        },
        git::get_latest_tag,
        io::error,
        sys::{
            cpu::{ARCH, cpu_arch_check},
            env::root_check,
        },
    },
};
use std::process::exit;

/// Only handles `.deb`, `.rpm`, and `binary` files. Updating via the `AUR` is handled separately.
/// This is due to installing via the `AUR` requiring to not be run as root.
pub fn update_via_package(package_type: &str) {
    let (green, blue, reset) = (COLORS.green, COLORS.blue, COLORS.reset);

    root_check();

    let latest_stable_release = get_latest_tag();

    cpu_arch_check(
        "You cannot update cmdcreate via this method using \
          CPU Architectures other than x86 or ARM variants!",
    );

    let target_arch = match ARCH {
        "x86_64" => {
            output!(
                "
                \nSelect target architecture:\n\
                1]{reset} x86_64 {blue}(64-bit)\n\
                2]{reset} i686 {blue}(32-bit)"
            );
            let arch_choice = input!("").trim().parse::<usize>().unwrap_or(1);
            if arch_choice == 2 { "i686" } else { "x86_64" }
        }
        "aarch64" => "aarch64",
        "armv7" => "armv7",
        _ => "i686",
    };

    let package_file_name =
        &format!("cmdcreate-{latest_stable_release}-linux-{target_arch}{package_type}");
    let temp_package_file_path = &format!("/tmp/{package_file_name}");

    let project_repo = PROJECT.repository;
    let package_file_download_path =
        &format!("{project_repo}/releases/latest/download/{package_file_name}");

    download_file_to_location_via_curl(temp_package_file_path, package_file_download_path);

    match package_type {
        ".deb" => run_shell_command!("dpkg -i {temp_package_file_path}"),
        ".rpm" => run_shell_command!("rpm -Uvh {temp_package_file_path}"),
        "-bin" => install_binary("-Dm755", temp_package_file_path, "/usr/bin/cmdcreate"),

        _ => error("Developer error: INVALID METHOD", Some(package_type)),
    }

    delete_file(temp_package_file_path);

    println!("\n{green}Update complete!");

    exit(0)
}
