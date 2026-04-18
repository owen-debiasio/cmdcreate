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
    commands::tools::cmdcreate_command_is_installed,
    logger::{Severity, log},
    output,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string, remove_from_file, write_to_file},
        io::error,
    },
};

pub fn command_is_in_favorites(command: &str) -> bool {
    let favorites_path = &PATHS.favorites;
    let favorites_file_contents = read_file_to_string(favorites_path);

    favorites_file_contents.contains(command)
}

pub fn favorite(action: &str, command: &str) {
    match action {
        "add" => add(command),
        "remove" => remove(command),

        _ => error("Invalid option:", action),
    }
}

fn add(command: &str) {
    let (blue, green, yellow, reset) = (COLORS.blue, COLORS.green, COLORS.yellow, COLORS.reset);

    log(
        &format!("commands/favorite::add(): Adding command \"{command}\" to favorites..."),
        Severity::Normal,
    );

    let favorites_path = &PATHS.favorites;

    if !cmdcreate_command_is_installed(command) {
        error(&format!("Command \"{command}\" does not exist!"), "")
    }

    if command_is_in_favorites(command) {
        println!("{yellow}Command {blue}\"{command}\"{yellow} is already in favorites.{reset}");

        return;
    }

    // Newline added to avoid issues
    let command_to_write = &format!("{command}\n");

    write_to_file(favorites_path, command_to_write, true).expect("Failed to write to file");

    command_favorite_addition_check(command_to_write);

    output!("{green}Command {blue}\"{command}\"{green} added to favorites.{reset}");
}

fn command_favorite_addition_check(command: &str) {
    log(
        "commands/favorite::command_favorite_addition_check(): \
        Determining command favorite addition status...",
        Severity::Normal,
    );

    if !command_is_in_favorites(command) {
        error(
            "Failed to remove command from favorites!",
            "Command is still located in configuration.",
        );
    }

    log(
        "commands/favorite::command_favorite_addition_check(): \
        Command has been added to favorites correctly...",
        Severity::Normal,
    );
}

fn remove(command: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/favorite::remove(): Removing command \"{command}\" from favorites..."),
        Severity::Normal,
    );

    if !command_is_in_favorites(command) {
        error("Command isn't in favorites:", command);
    }

    let favorites_path = &PATHS.favorites;

    remove_from_file(favorites_path, command).expect("Failed to remove contents from file");

    command_favorite_removed_check(command);

    output!("{green}Command {blue}\"{command}\"{green} removed from favorites.{reset}");
}

fn command_favorite_removed_check(command: &str) {
    log(
        "commands/favorite::command_favorite_removed_check(): \
        Determining command favorite removal status...",
        Severity::Normal,
    );

    if command_is_in_favorites(command) {
        error(
            "Failed to remove command from favorites!",
            "Command is still located in configuration.",
        );
    }

    log(
        "commands/favorite::command_favorite_removed_check(): \
        Command has been removed from favorites correctly...",
        Severity::Normal,
    );
}
