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
    commands::core::{create::create, favorite::main::favorite},
    core::logger::{consts::Severity, main::log},
    output,
    utils::{colors::COLORS, fs::core::read_write::read_file_to_string, io::error},
};

pub fn import(command_import_file: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "commands::core::backup_actions::import::import(): \
            Importing commands from file: \"{command_import_file}\"..."
        ),
        Severity::Normal,
    );

    let command_import_file_contents = read_file_to_string(command_import_file);

    if command_import_file_contents.trim().is_empty() {
        error("Import file is empty or unreadable.", None);
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
            "Installing command: \"{green}{retrieved_command_name}{blue}\"",
            true
        );

        create(retrieved_command_name, &command_contents, false);

        if command_favorite_status {
            favorite("add", retrieved_command_name);
        }
    }

    output!("\n{green}Successfully imported commands.{reset}", true);
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{
            core::backup_actions::{export::export, import::import},
            tools::{cmdcreate_command_is_installed, tests::TestCommand},
        },
        run_shell_command,
        utils::fs::core::creation::{create_folder, delete_folder},
    };

    fn export_commands(directory: &str) {
        create_folder(directory);

        export(directory);
    }

    #[test]
    #[ignore = "Must be called manually to avoid conflicts with other tests."]
    fn imported_commands_are_located_in_install_dir() {
        let test_name = "imported_commands_are_located_in_install_dir";
        TestCommand::create_group(test_name, true);

        let temp_dir_name = "export_file_contains_command_with_contents_temp";

        export_commands(temp_dir_name);

        let file_to_import = &format!("{temp_dir_name}/export.cmdcreate");

        import(file_to_import);

        for command_i in 1..=3 {
            let command_is_installed =
                cmdcreate_command_is_installed(&format!("{test_name}_{command_i}"));
            assert!(command_is_installed);
        }

        delete_folder(temp_dir_name);

        TestCommand::remove_group(test_name);
    }

    #[test]
    #[ignore = "Must be called manually to avoid conflicts with other tests."]
    fn imported_commands_run_correctly() {
        let test_name = "imported_commands_run_correctly";
        TestCommand::create_group(test_name, true);

        let temp_dir_name = "imported_commands_run_correctly_temp";

        export_commands(temp_dir_name);

        let file_to_import = &format!("{temp_dir_name}/export.cmdcreate");

        import(file_to_import);

        for command_i in 1..=3 {
            let command = &format!("{test_name}_{command_i}");
            let command_runs_correctly = run_shell_command!(bool: "{command}");
            assert!(command_runs_correctly);
        }

        delete_folder(temp_dir_name);

        TestCommand::remove_group(test_name);
    }
}
