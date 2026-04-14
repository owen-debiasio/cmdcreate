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
        favorite::{command_is_in_favorites, favorite},
        remove::remove,
        tools::{cmdcreate_command_is_installed, get_installed_commands},
    },
    logger::{Severity, log},
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

    let (command_install_location, installed_commands) = (
        &PATHS.command_installation_directory,
        get_installed_commands(),
    );

    if installed_commands.is_empty() {
        return;
    }

    cmdcreate_command_is_installed(old_command_name);

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
            "commands/rename::rename(): Renaming command \"{old_command_name}\" \
            to \"{new_renamed_command_name}\"..."
        ),
        Severity::Normal,
    );

    let mut check_favorites_in_repair: bool = false;
    if command_is_in_favorites(old_command_name) {
        check_favorites_in_repair = true;

        favorite("remove", old_command_name);
        favorite("add", new_renamed_command_name);
    }

    // I should probably make a function to move files
    run_shell_command(&format!(
        "mv \
        {command_install_location}{old_command_name} \
        {command_install_location}{new_renamed_command_name}"
    ));

    command_rename_success(
        old_command_name,
        new_renamed_command_name,
        command_install_location,
        check_favorites_in_repair,
    );

    println!(
        "{green}Successfully renamed command \
        {blue}\"{old_command_name}\" \
        to {blue}\"{new_renamed_command_name}\"{reset}"
    );
}

fn command_rename_success(
    old_name: &str,
    new_name: &str,
    path_of_old_command: &str,
    check_favorite_status: bool,
) {
    log(
        "commands/rename::command_rename_success(): \
        Determining command creation status...",
        Severity::Normal,
    );

    // Avoid 'cmdcreate_command_is_installed(path_of_old_command)'
    // because it assumes command was already deleted.
    // It is checked manually instead.
    if !path_exists(path_of_old_command) {
        error("Failed to rename command!", "Old command is still present.");
    }

    if !cmdcreate_command_is_installed(new_name) {
        error("Failed to rename command!", "New command does not exist.");
    }

    if check_favorite_status {
        if command_is_in_favorites(old_name) {
            error(
                "Failed to rename command!",
                "Old command still remains in favorites.",
            );
        }

        if !command_is_in_favorites(new_name) {
            error(
                "Failed to rename command!",
                "New command is not in the favorites file.",
            );
        }
    }

    log(
        "commands/rename::command_rename_success(): \
        Command has been renamed correctly...",
        Severity::Normal,
    );
}
