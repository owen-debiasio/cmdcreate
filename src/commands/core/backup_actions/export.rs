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
    commands::{core::create::NEW_COMMAND_HEADER, tools::get_installed_commands},
    core::logger::{consts::Severity, main::log},
    output,
    utils::{
        colors::COLORS,
        fs::{
            core::{
                creation::create_file,
                read_write::{read_file_to_string, write_to_file},
            },
            paths::{PATHS, path_exists},
        },
        io::error,
        sys::env::root_check,
    },
};

pub fn export(path: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    // if !(path.starts_with('/') || path.starts_with('~')) {
    //     error("Not a valid path:", Some(path))
    // }

    if path.starts_with('/') && !path.starts_with("/home") {
        root_check();
    }

    if !path_exists(path) {
        error("Not a valid path:", Some(path))
    }

    let path_of_file_to_export_to = &format!("{path}/export.cmdcreate").replace("//", "/");

    log(
        &format!(
            "commands/core/backup_actions/export::export(): \
            Exporting commands to: \"{path_of_file_to_export_to}\"..."
        ),
        Severity::Normal,
    );

    log(
        "commands/core/backup_options/export::export(): \
        Creating export file...",
        Severity::Normal,
    );

    create_file(path_of_file_to_export_to);

    let installed_commands = get_installed_commands();

    for retrieved_command in installed_commands {
        log(
            &format!(
                "commands::core::backup_actions::export::export(): \
                Exporting command: \"{retrieved_command}\"..."
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

        let favorites_file_contents = read_file_to_string(&PATHS.favorites);

        let data = if favorites_file_contents.contains(&retrieved_command) {
            format!("{retrieved_command} | {final_contents_of_command} | favorite\n")
        } else {
            format!("{retrieved_command} | {final_contents_of_command}\n")
        };

        log(
            &format!(
                "commands::core::backup_actions::export::export(): \
                Writing data to file: \"{data}\"..."
            ),
            Severity::Normal,
        );

        write_to_file(path_of_file_to_export_to, &data, true);
    }

    output!(
        "{green}Successfully exported commands to: \
        {blue}\"{path_of_file_to_export_to}\"{green}.{reset}",
        true
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{core::backup_actions::export::export, tools::tests::TestCommand},
        utils::fs::{
            core::{
                creation::{create_folder, delete_folder},
                read_write::read_file_to_string,
            },
            paths::path_exists,
        },
    };

    #[test]
    #[ignore = "Must be called manually to avoid conflicts with other tests."]
    fn export_file_is_created() {
        let test_name = "export_file_is_created";

        TestCommand::create_group(test_name, false);

        let temp_dir_name = "export_file_is_created_temp_destination";

        create_folder(temp_dir_name);

        export(temp_dir_name);

        let export_file_name = &format!("{temp_dir_name}/export.cmdcreate");

        assert!(path_exists(export_file_name));

        TestCommand::remove_group(test_name);
        delete_folder(temp_dir_name);
    }

    #[test]
    #[ignore = "Must be called manually to avoid conflicts with other tests."]
    fn export_file_contains_command_with_contents() {
        let test_name = "export_file_contains_command_with_contents";

        TestCommand::create_group(test_name, true);

        let temp_dir_name = "export_file_contains_command_with_contents_temp";

        create_folder(temp_dir_name);

        export(temp_dir_name);

        let export_file_name = &format!("{temp_dir_name}/export.cmdcreate");

        let export_file_contents = read_file_to_string(export_file_name);

        for command_i in 1..=3 {
            let command = &format!("{test_name}_{command_i}");

            let contents_to_match = if command.ends_with("_2") {
                &format!("{command} | true | favorite")
            } else {
                &format!("{command} | true")
            };

            let command_is_exported_correctly = export_file_contents.contains(contents_to_match);

            assert!(command_is_exported_correctly);
        }

        TestCommand::remove_group(test_name);

        delete_folder(temp_dir_name);
    }
}
