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
    commands::{create::NEW_COMMAND_HEADER, tools::get_installed_commands},
    logger::{Severity, log},
    utils::{
        colors::COLORS,
        fs::{PATHS, create_file, read_file_to_string, write_to_file},
    },
};

pub fn export(path: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let path_of_file_to_export_to = &format!("{path}/export.cmdcreate");

    log(
        &format!(
            "commands/export::export(): Exporting commands to: \"{path_of_file_to_export_to}\"..."
        ),
        Severity::Normal,
    );

    log(
        "commands/export::export(): Creating export file...",
        Severity::Normal,
    );

    create_file(path_of_file_to_export_to).expect("Failed to create file");

    let installed_commands = get_installed_commands();

    for installed_command in installed_commands {
        if let Some(command_stem) = installed_command.file_stem() {
            let retrieved_command = command_stem.to_string_lossy();

            log(
                &format!(
                    "commands/export::export(): Exporting command: \"{retrieved_command}\"..."
                ),
                Severity::Normal,
            );

            let path_of_command = &format!(
                "{}{retrieved_command}",
                PATHS.command_installation_directory
            );

            let original_contents_of_command = read_file_to_string(path_of_command);

            // The escape thing is "[|" cause backslashes don't work for some reason
            let contents_of_command_un_escaped = original_contents_of_command.replace('|', "[|");

            // Remove the header because
            // A. It will make the exported file look ugly and cause issues
            // B. It will be created again anyway
            let final_contents_of_command =
                contents_of_command_un_escaped.replace(NEW_COMMAND_HEADER, "");

            let data = if read_file_to_string(&PATHS.favorites).contains(retrieved_command.as_ref())
            {
                format!("{retrieved_command} | {final_contents_of_command} | favorite\n")
            } else {
                format!("{retrieved_command} | {final_contents_of_command}\n")
            };

            log(
                &format!("commands/export::export(): Writing data to file: \"{data}\"..."),
                Severity::Normal,
            );

            write_to_file(path_of_file_to_export_to, &data, true).expect("Failed to write to file");
        }
    }

    println!(
        "{green}Successfully exported commands to: {blue}\"{path_of_file_to_export_to}\"{green}.{reset}",
    );
}
