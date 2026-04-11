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
    commands::{favorite::command_is_in_favorites, tools::get_installed_commands},
    configs::load_configuration,
    logger::{Severity, log},
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn list() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let installed_commands = get_installed_commands();

    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_commands.len()
    );

    for command in installed_commands {
        let command_name = command.file_stem().unwrap_or_default().to_string_lossy();

        log(
            &format!(
                "commands/list::list(): Current command: \"{command_name}\" (favorited={})",
                command_is_in_favorites(&command_name)
            ),
            Severity::Normal,
        );

        // The default is a star
        let favorite_command_identifier =
            load_configuration("appearance", "favorite_indicator", "\u{2605}");
        if command_is_in_favorites(&command_name) {
            println!("{favorite_command_identifier} {command_name}");

            continue;
        }

        let favorites_path = &PATHS.favorites;
        let favorites_file_contents = read_file_to_string(favorites_path);

        if favorites_file_contents.is_empty() {
            println!("{command_name}");
            continue;
        }

        let favorite_command_identifier_length = favorite_command_identifier.len();

        let output_spacing = " ".repeat(favorite_command_identifier_length);
        println!("{output_spacing} {command_name}");
    }
}
