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
        clean::clean,
        create::create,
        display::display,
        edit::edit,
        export::export,
        favorite::favorite,
        import::import,
        list::list,
        remove::remove,
        rename::rename,
        search::search,
        update::{check, update},
    },
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::error,
        net::is_offline,
        sys::{arguments_force_actions, run_shell_command},
    },
    version::print_version_info,
};

pub fn parse(supplied_command: &str, supplied_arguments: &[String]) {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    let argument_index = |i| supplied_arguments.get(i).map(String::as_str);

    log(
        &format!("parse::parse(): Parsing command: {supplied_command}"),
        0,
    );

    // Like all of these lines are the fucking same

    match supplied_command {
        "create" => match (argument_index(1), argument_index(2)) {
            (Some(command), Some(contents)) => create(command, contents, true),
            _ => println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}"),
        },

        "remove" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}"),
            |cmd| remove(cmd, arguments_force_actions()),
        ),

        "edit" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}edit {yellow}<command>{reset}"),
            edit,
        ),

        "search" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}"),
            search,
        ),

        "display" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}"),
            display,
        ),

        "rename" => match (argument_index(1), argument_index(2)) {
            (Some(command), Some(new_name)) => rename(command, new_name),
            _ => println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}"),
        },

        "favorite" => match (argument_index(1), argument_index(2)) {
            (Some(mode @ ("add" | "remove")), Some(command)) => favorite(mode, command),
            _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
        },

        "list" => list(),
        "check" => check(),
        "update" => update(),
        "clean" => clean(),

        "import" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}"),
            import,
        ),

        "export" => argument_index(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}export {yellow}<output dir>{reset}"),
            export,
        ),

        "--version" | "-v" => print_version_info(),

        "--license" | "-l" => {
            if path_exists(&PATHS.license) {
                /*
                read_file_to_string("/usr/share/licenses/cmdcreate/LICENSE")
                doesn't work and idk why???
                */
                run_shell_command(&format!("cat {}", PATHS.license));
            } else {
                error(
                    "License has not been installed. Find it here:",
                    "https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE",
                )
            }
        }

        "--changelog" | "-c" => {
            if is_offline() {
                error("You need internet to retrieve the changelog.", "")
            }

            log("main::main(): Displaying changelog...", 0);

            run_shell_command(
                "curl -L https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/changes.md",
            );
        }

        _ if supplied_command.starts_with('-') => {
            error("Invalid argument:", supplied_command);
        }

        _ => {
            error("Invalid command:", supplied_command);
        }
    }
}
