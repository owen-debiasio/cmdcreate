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
        config::config, create::create, display::display, doc::doc, edit::edit, export::export,
        favorite::favorite, import::import, list::list, remove::remove, rename::rename,
        search::search, update::update,
    },
    logger::{Severity, log},
    utils::{
        colors::COLORS,
        io::error,
        sys::{arguments::arguments_force_actions, env::root_check},
    },
    version::{check, print_version_info},
};

macro_rules! validate_args {
    ($command:expr, $amount_of_arguments_given:expr, $amount_of_arguments_needed:expr, $command_usage:expr, $additional_args:expr) => {
        if $amount_of_arguments_given.len() <= $amount_of_arguments_needed {
            let (blue, yellow, red, green, reset) = (
                COLORS.blue,
                COLORS.yellow,
                COLORS.red,
                COLORS.green,
                COLORS.reset,
            );

            let include_additional_flags = if !$additional_args.is_empty() {
                &format!("{reset}[{green}{}{reset}] ", $additional_args)
            } else {
                ""
            };

            println!(
                "Usage:\ncmdcreate {blue}{} {include_additional_flags}{yellow}{}{red}{reset}",
                $command, $command_usage
            );

            return;
        }
    };
}

#[allow(clippy::too_many_lines)]
pub fn parse(supplied_command: &str, supplied_arguments: &[String]) {
    log(
        &format!("parse::parse(): Parsing command: {supplied_command}"),
        Severity::Normal,
    );

    let positional_args: Vec<&str> = supplied_arguments
        .iter()
        .filter(|supplied_argument| !supplied_argument.starts_with('-'))
        .map(String::as_str)
        .collect();

    let argument_index = |index| positional_args.get(index).copied();

    match supplied_command {
        "create" => {
            validate_args!(
                supplied_command,
                supplied_arguments,
                2,
                "<command> <contents>",
                "-i/--in_editor"
            );

            root_check();

            let command_name = argument_index(1).unwrap_or("");
            let command_contents = argument_index(2).unwrap_or("");

            create(command_name, command_contents, true);
        }
        "rename" => {
            validate_args!(
                supplied_command,
                supplied_arguments,
                2,
                "<command> <new name>",
                ""
            );

            root_check();

            let old_command_name = argument_index(1).unwrap();
            let renamed_command_name = argument_index(2).unwrap();

            rename(old_command_name, renamed_command_name);
        }
        "favorite" => {
            validate_args!(
                supplied_command,
                supplied_arguments,
                2,
                "<add/remove> <command>",
                ""
            );

            root_check();

            let command_operation = argument_index(1).unwrap(); // Either "add" or "remove"
            let name_of_command = argument_index(2).unwrap();

            favorite(command_operation, name_of_command);
        }

        "remove" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<command>", "");

            root_check();

            for index in 1..=supplied_arguments.len() {
                if let Some(command_to_remove) = argument_index(index) {
                    remove(command_to_remove, arguments_force_actions());
                }
            }
        }
        "edit" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<command>", "");

            root_check();

            let command_to_edit = argument_index(1).unwrap();

            edit(command_to_edit);
        }
        "search" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<command>", "");

            let command_to_search_for = argument_index(1).unwrap();

            search(command_to_search_for);
        }
        "display" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<command>", "");

            let command_to_display_contents_of = argument_index(1).unwrap();

            display(command_to_display_contents_of);
        }
        "import" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<input file>", "");

            root_check();

            let file_to_import_commands_from = argument_index(1).unwrap();

            import(file_to_import_commands_from);
        }
        "export" => {
            validate_args!(supplied_command, supplied_arguments, 1, "<output dir>", "");

            let destination_of_exported_commands = argument_index(1).unwrap();

            export(destination_of_exported_commands);
        }

        "config" => {
            // Instead of showing the usage, default to the list of available
            // documentation
            if supplied_arguments.len() == 1 {
                validate_args!(
                    supplied_command,
                    supplied_arguments,
                    3,
                    "<help/example/add/remove/edit/display> <category> <value(=\"setting\")>",
                    ""
                );
                return;
            }

            root_check();

            let config_mode = argument_index(1).unwrap_or("");
            let config_category = argument_index(2).unwrap_or("");
            let config_value = argument_index(3).unwrap_or("");

            config(config_mode, config_category, config_value);
        }
        "doc" => {
            validate_args!(
                supplied_command,
                supplied_arguments,
                1,
                "<list>/<information>",
                ""
            );

            let information_to_get = argument_index(1).unwrap();

            doc(information_to_get);
        }

        "list" => list(),
        "check" => check(),
        "update" => update(),

        "--version" | "-v" => print_version_info(),

        _ if supplied_command.starts_with('-') => error("Invalid argument:", supplied_command),
        _ => error("Invalid command:", supplied_command),
    }
}
