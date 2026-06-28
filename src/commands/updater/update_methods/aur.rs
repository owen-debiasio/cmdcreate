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
    output, run_shell_command,
    utils::{
        colors::COLORS,
        fs::{
            core::creation::delete_folder,
            paths::{CMDCREATE_BINARY_PATH, path_exists},
        },
        io::{ask_for_confirmation, error},
        sys::env::running_as_root,
    },
};
use std::process::exit;

pub fn update_via_aur() {
    let (blue, magenta, green) = (COLORS.blue, COLORS.magenta, COLORS.green);

    if running_as_root() {
        error(
            "Please de-escalate from root to update using this method!",
            Some("You can't use the AUR when running from root."),
        )
    }

    let package_name: &str =
        if ask_for_confirmation("Do you want to install the latest git?", false) {
            "cmdcreate-git"
        } else {
            "cmdcreate"
        };

    output!(
        "{blue}Updating... {magenta}Please wait as this will take a while",
        true
    );

    let cmdcreate_bin_path = &CMDCREATE_BINARY_PATH.to_string();

    let command_to_install = format!(
        "set -e

        sudo rm -rf {cmdcreate_bin_path} /tmp/cmdcreate_aur_tmp

        git clone https://aur.archlinux.org/{package_name}.git /tmp/cmdcreate_aur_tmp

        cd /tmp/cmdcreate_aur_tmp

        makepkg -si --noconfirm -- --overwrite /usr/bin/cmdcreate
        ",
    );

    run_shell_command!("{command_to_install}");

    delete_folder("/tmp/cmdcreate_aur_tmp");

    if !path_exists("/usr/bin/cmdcreate") {
        error("Update failed:", Some("Binary not found."))
    }

    output!("\n{green}Update complete!");

    exit(0)
}
