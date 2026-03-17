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
    commands::{
        remove::remove,
        tools::{determine_command_is_installed, get_installed_commands},
    },
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::{ask_for_confirmation, error},
        sys::run_shell_command,
    },
};

pub fn rename(old_command_name: &str, new_renamed_command_name: &str) {
    let (blue, red, yellow, green, reset) = (
        COLORS.blue,
        COLORS.red,
        COLORS.yellow,
        COLORS.green,
        COLORS.reset,
    );

    let (command_install_location, installed_commands) =
        (&PATHS.install_dir, get_installed_commands());

    if installed_commands.is_empty() {
        return;
    }

    determine_command_is_installed(old_command_name);

    let new_command_install_location =
        &format!("{command_install_location}/{new_renamed_command_name}");

    if path_exists(new_command_install_location) {
        let question = &format!(
            "{red}The new name ({yellow}{new_renamed_command_name}{red}) is already installed! \
            Do you want to delete it?"
        );

        if ask_for_confirmation(question, false) {
            remove(new_renamed_command_name, true);
        } else {
            error("You need to remove the old command before proceeding!", "");
        }
    }

    log(
        &format!(
            "commands/rename::rename(): Renaming command \"{old_command_name}\" to \"{new_renamed_command_name}\"..."
        ),
        0,
    );

    // I should probably make a function to move files
    run_shell_command(&format!(
        "mv {command_install_location}{old_command_name} {command_install_location}{new_renamed_command_name}"
    ));

    println!(
        "{green}Successfully renamed command {blue}\"{old_command_name}\" to {blue}\"{new_renamed_command_name}\"{reset}"
    );
}
