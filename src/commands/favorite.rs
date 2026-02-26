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
    commands::tools::command_is_installed,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{read_file_to_string, remove_from_file, write_to_file, PATHS},
        io::error,
    },
};

pub fn favorite(mode: &str, command: &str) {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    match mode {
        "add" => add(command),
        "remove" => remove(command),

        _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
    }
}

fn add(cmd: &str) {
    let (blue, green, yellow, reset) = (COLORS.blue, COLORS.green, COLORS.yellow, COLORS.reset);

    log(
        &format!("commands/favorite::add(): Adding command \"{cmd}\" to favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    command_is_installed(cmd);

    if read_file_to_string(favorites_path)
        .expect("Failed to retrieve favorites")
        .lines()
        .any(|c| c == cmd)
    {
        println!("{yellow}Command {blue}\"{cmd}\"{yellow} is already in favorites.{reset}");

        return;
    }

    write_to_file(favorites_path, &format!("{cmd}\n"), true).expect("Failed to write to file");

    println!("{green}Command {blue}\"{cmd}\"{green} added to favorites.{reset}");
}

fn remove(cmd: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/favorite::remove(): Removing command \"{cmd}\" from favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    if !read_file_to_string(favorites_path)
        .expect("Failed to retrieve favorites")
        .lines()
        .any(|c| c == cmd)
    {
        error("Command isn't in favorites:", cmd);
    }

    remove_from_file(favorites_path, cmd).expect("Failed to remove contents from file");

    println!("{green}Command {blue}\"{cmd}\"{green} removed from favorites.{reset}");
}
