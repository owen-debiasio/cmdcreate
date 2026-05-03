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
    commands::{create::create, favorite::favorite as add_favorite},
    logger::{Severity, log},
    output,
    utils::{colors::COLORS, fs::core::read_file_to_string, io::error},
};

pub fn import(command_import_file: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "commands/import::import(): Importing commands from file: \"{command_import_file}\"..."
        ),
        Severity::Normal,
    );

    let command_import_file_contents = read_file_to_string(command_import_file);

    if command_import_file_contents.trim().is_empty() {
        error("Import file is empty or unreadable.", "");
    }

    for entry in command_import_file_contents.replace("[|", "|").lines() {
        let parts_of_exported_entry: Vec<&str> = entry.split('|').map(str::trim).collect();

        if entry.trim().is_empty() && !parts_of_exported_entry.is_empty() {
            continue;
        }

        let retrieved_command_name = parts_of_exported_entry.first().unwrap();
        let mut command_contents = String::new();
        let mut command_favorite_status = false;

        for part in parts_of_exported_entry.iter().skip(1) {
            if *part == "favorite" {
                command_favorite_status = true;
            } else {
                if command_contents.is_empty() {
                    command_contents.push('\n');
                }

                command_contents.push_str(part);
            }
        }

        output!(
            "{blue}Installing command: \"{green}{retrieved_command_name}{reset}\"",
            true
        );

        create(retrieved_command_name, &command_contents, false);

        if command_favorite_status {
            add_favorite("add", retrieved_command_name);
        }
    }

    output!("\n{green}Successfully imported commands.{reset}", true);
}
