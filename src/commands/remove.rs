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
    commands::{favorite::favorite, tools::cmdcreate_command_is_installed},
    logger::{Severity, log},
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, path_exists, read_file_to_string},
        io::{ask_for_confirmation, error},
    },
};

pub fn remove(command: &str, force_removal_of_command: bool) {
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    cmdcreate_command_is_installed(command);

    if !force_removal_of_command {
        let question = &format!(
            "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
        );

        ask_for_confirmation(question, true);
    }

    if read_file_to_string(&PATHS.favorites).contains(command) {
        favorite("remove", command);
    }

    log(
        &format!("commands/remove::remove(): Removing command \"{command}\"..."),
        Severity::Normal,
    );

    let path_of_command_to_remove = &format!("{}{command}", PATHS.command_installation_directory);

    delete_file(path_of_command_to_remove).expect("Failed to delete command");

    command_removal_success(path_of_command_to_remove);

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}

fn command_removal_success(path_of_command: &str) {
    log(
        "commands/remove::command_removal_success(): \
        Determining command removal status...",
        Severity::Normal,
    );

    // Avoid 'cmdcreate_command_is_installed(command_to_check)'
    // because it assumes command was already deleted.
    // It is checked manually instead.
    if path_exists(path_of_command) {
        error("Failed to remove command!", "Failed to delete script.");
    }

    log(
        "commands/remove::command_creation_success(): \
        Command has been removed correctly...",
        Severity::Normal,
    );
}
