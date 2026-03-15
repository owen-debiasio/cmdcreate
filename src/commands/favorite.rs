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
    commands::tools::determine_command_is_installed,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string, remove_from_file, write_to_file},
        io::error,
    },
};

pub fn favorite(action: &str, command: &str) {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    match action {
        "add" => add(command),
        "remove" => remove(command),

        _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
    }
}

// Almost identical to remove()
fn add(command: &str) {
    let (blue, green, yellow, reset) = (COLORS.blue, COLORS.green, COLORS.yellow, COLORS.reset);

    log(
        &format!("commands/favorite::add(): Adding command \"{command}\" to favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    determine_command_is_installed(command);

    if read_file_to_string(favorites_path)
        .lines()
        .any(|command_| command_ == command)
    {
        println!("{yellow}Command {blue}\"{command}\"{yellow} is already in favorites.{reset}");

        return;
    }

    // Newline added to avoid issues
    let command_to_write = &format!("{command}\n");

    write_to_file(favorites_path, command_to_write, true).expect("Failed to write to file");

    println!("{green}Command {blue}\"{command}\"{green} added to favorites.{reset}");
}

// Almost identical to add()
fn remove(command: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/favorite::remove(): Removing command \"{command}\" from favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    if !read_file_to_string(favorites_path)
        .lines()
        .any(|command_| command_ == command)
    {
        error("Command isn't in favorites:", command);
    }

    remove_from_file(favorites_path, command).expect("Failed to remove contents from file");

    println!("{green}Command {blue}\"{command}\"{green} removed from favorites.{reset}");
}
