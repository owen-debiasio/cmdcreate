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
    commands::{edit::edit as edit_command, remove::remove, tools::cmdcreate_command_is_installed},
    logger::{Severity, log},
    utils::{
        colors::COLORS,
        fs::{PATHS, create_file, overwrite_file, read_file_to_string},
        io::error,
        sys::{args_contains, run_shell_command},
    },
};

// Uniquely identify commands created by cmdcreate from
// commands that may also show up in /usr/local/bin/
pub static NEW_COMMAND_HEADER: &str = "# Created by cmdcreate\n";

pub fn create(
    command_to_create: &str,
    contents_of_new_command: &str,
    run_this_function_verbose: bool,
) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let verbose_creation_message = if run_this_function_verbose {
        ", and being verbose..."
    } else {
        "..."
    };

    let log_message = &format!(
        "commands/create::create(): Creating command \"{command_to_create}\": \
        With contents \"{contents_of_new_command}\"{verbose_creation_message}",
    );

    log(log_message, Severity::Normal);

    let full_contents_of_new_command = &format!("{NEW_COMMAND_HEADER}{contents_of_new_command}");

    let path_to_command = &format!(
        "{}{command_to_create}",
        PATHS.command_installation_directory
    );

    log(
        &format!("commands/create::create(): Command path: \"{path_to_command}\""),
        Severity::Normal,
    );

    if contents_of_new_command.is_empty() {
        error("The contents of your command can not be empty.", "");
    }

    create_file(path_to_command).expect("Failed to create initial command file");

    overwrite_file(path_to_command, full_contents_of_new_command).expect("Failed to write");

    let user_edits_contents_in_editor = args_contains("--in_editor") || args_contains("-i");

    if user_edits_contents_in_editor {
        edit_command(command_to_create);
    }

    log(
        "commands/create::create(): Activating command...",
        Severity::Normal,
    );

    run_shell_command(&format!("chmod +x {path_to_command}"));

    command_creation_success(command_to_create, contents_of_new_command, path_to_command);

    if run_this_function_verbose {
        println!(
            "\n{green}Success! Created command: \
        {blue}\"{command_to_create}\"{reset}"
        );
    }
}

fn command_creation_success(
    command_to_check: &str,
    user_chosen_contents: &str,
    installed_command_path: &str,
) {
    log(
        "commands/create::command_creation_success(): \
        Determining command creation status...",
        Severity::Normal,
    );

    let installed_command_contents = read_file_to_string(installed_command_path);

    if !cmdcreate_command_is_installed(command_to_check) {
        clean_from_failure(installed_command_path);

        error("Failed to create command!", "Failed to create script.");
    }

    if !installed_command_contents.contains(user_chosen_contents) {
        clean_from_failure(installed_command_path);

        error(
            "Failed to create command!",
            "Failed to write command contents.",
        )
    }

    log(
        "commands/create::command_creation_success(): \
        Command has been created correctly...",
        Severity::Normal,
    );
}

fn clean_from_failure(command_name: &str) {
    log(
        "commands/create::clean_from_failure(): \
        Starting on-fail cleanup...",
        Severity::Normal,
    );

    remove(command_name, true);

    log(
        "commands/create::clean_from_failure(): \
        Cleaned up...",
        Severity::Normal,
    );
}
