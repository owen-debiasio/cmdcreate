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
    usage::debug_usage,
    utils::{
        colors::COLORS,
        fs::{delete_file, init_git_fs, read_file_to_string, PATHS},
        io::error,
        sys::args_forced,
    },
    VERSION,
};

pub fn parse(cmd: &str, args: &[String]) {
    let (green, blue, yellow, reset) = (COLORS.green, COLORS.blue, COLORS.yellow, COLORS.reset);

    let arg = |i| args.get(i).map(String::as_str);

    log(&format!("parse::parse(): Parsing command: {cmd}"), 0);

    match cmd {
        "create" => match (arg(1), arg(2)) {
            (Some(command), Some(contents)) => create(command, contents, true),
            _ => println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}"),
        },

        "remove" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}"),
            |cmd| remove(cmd, args_forced()),
        ),

        "edit" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}edit {yellow}<command>{reset}"),
            edit,
        ),

        "search" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}"),
            search,
        ),

        "display" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}"),
            display,
        ),

        "rename" => match (arg(1), arg(2)) {
            (Some(command), Some(new_name)) => rename(command, new_name),
            _ => println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}"),
        },

        "favorite" => match (arg(1), arg(2)) {
            (Some(mode @ ("add" | "remove")), Some(command)) => favorite(mode, command),
            _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
        },

        "list" => list(),
        "check" => check(),
        "update" => update(),
        "clean" => clean(),

        "import" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}"),
            import,
        ),

        "export" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}export {yellow}<output dir>{reset}"),
            export,
        ),

        "--version" | "-v" => {
            println!("cmdcreate {VERSION}\n(C) 2026 Owen Debiasio; Licensed under MIT");
        }

        "--get_offline_files" | "-g" => {
            println!("Downloading offline files...");

            log("main::main(): Retrieval of offline files requested...", 0);

            init_git_fs().expect("Failed to initialize git files");

            log("main::main(): Retrieved offline files...", 0);

            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            log("main::main(): Removing offline files...", 0);

            println!("Removing files...");

            delete_file(&PATHS.changelog).expect("Failed to delete changelog");
            delete_file(&PATHS.license).expect("Failed to delete license");

            println!("{green}Files removed successfully.{reset}");
        }

        "--license" | "-l" => {
            log("main::main(): Displaying license...", 0);
            println!("{:?}", read_file_to_string(&PATHS.license));
        }

        "--changelog" | "-c" => {
            log("main::main(): Displaying changelog...", 0);
            println!(
                "{}",
                read_file_to_string(&PATHS.changelog)
                    .expect("Failed to read changelog")
                    .trim()
            );
        }

        "--debugging" | "-d" => debug_usage(),

        _ if cmd.starts_with('-') => {
            error("Invalid argument:", cmd);
        }

        _ => {
            error("Invalid command:", cmd);
        }
    }
}
