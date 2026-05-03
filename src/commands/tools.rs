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

use std::{fs::read_dir, path::PathBuf};

use crate::{
    commands::create::NEW_COMMAND_HEADER,
    logger::{Severity, log},
    utils::{
        fs::{PATHS, path_exists, read_file_to_string},
        io::error,
    },
};

pub fn cmdcreate_command_is_installed(command_to_find: &str) -> bool {
    let command_install_path =
        &format!("{}{command_to_find}", PATHS.command_installation_directory);

    // Using both path_exists() and get_installed_commands().contains()
    // Will help the accuracy if one method doesn't work for some reason.
    if path_exists(command_install_path)
        || get_installed_commands().contains(&command_to_find.to_string())
    {
        log(
            &format!(
                "commands/tools::command_is_installed(): \
                Command \"{command_to_find}\" is installed... Continuing..."
            ),
            Severity::Normal,
        );

        return true;
    }

    error(
        &format!("Command \"{command_to_find}\" is not installed"),
        "",
    );
}

pub fn get_installed_commands() -> Vec<String> {
    log(
        "commands/tools::get_installed_commands(): Getting installed commands...",
        Severity::Normal,
    );

    let command_install_directory = &PATHS.command_installation_directory;

    let mut retrieved_commands_pathbuf: Vec<PathBuf> = read_dir(command_install_directory)
        .unwrap_or_else(|_| error("Error: Failed to read install directory!", ""))
        .flatten()
        .map(|entry_in_index| entry_in_index.path())
        .filter(|path_to_command| path_to_command.is_file())
        .collect();

    // Remove commands not created by cmdcreate
    // (commands that don't contain the command header)
    // (See src/commands/create.rs)
    retrieved_commands_pathbuf.retain(|command_path| {
        let path_of_command_found = &command_path.to_string_lossy();
        let file_contents = read_file_to_string(path_of_command_found);

        file_contents.contains(NEW_COMMAND_HEADER)
    });

    // Manually convert the original Vec<Pathbuf> to
    // Vec<String> because for me, it makes it easier
    // to deal with retrieving the installed commands.
    let mut retrieved_commands: Vec<String> = Vec::new();
    for command in retrieved_commands_pathbuf {
        let command_vec_addition_full = command.to_str().unwrap().to_string();

        // Remove "/usr/local/bin/" to just provide
        // the names of the commands. They are all
        // installed in the same directory anyway.
        let command_to_add = command_vec_addition_full.replace(command_install_directory, "");

        retrieved_commands.push(command_to_add);
    }

    if retrieved_commands.is_empty() {
        error("No commands are installed.", "");
    }

    retrieved_commands
}
