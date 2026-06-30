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
    commands::core::favorite::main::command_is_in_favorites,
    core::logger::{consts::Severity, main::log},
    output,
    utils::{
        colors::COLORS,
        fs::{core::read_write::remove_from_file, paths::PATHS},
        io::error,
    },
};

pub fn remove(command: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "commands/favorite/remove_favorite::remove(): Removing command \"{command}\" from favorites..."
        ),
        Severity::Normal,
    );

    if !command_is_in_favorites(command) {
        error("Command isn't in favorites:", Some(command));
    }

    let favorites_path = &PATHS.favorites;

    remove_from_file(favorites_path, command);

    command_favorite_removed_check(command);

    output!(
        "{green}Command {blue}\"{command}\"{green} removed from favorites.{reset}",
        true
    );
}

fn command_favorite_removed_check(command: &str) {
    log(
        "commands/favorite/remove_favorite::command_favorite_removed_check(): \
        Determining command favorite removal status...",
        Severity::Normal,
    );

    if command_is_in_favorites(command) {
        error(
            "Failed to remove command from favorites!",
            Some("Command is still located in configuration."),
        );
    }

    log(
        "commands/favorite/remove_favorite::command_favorite_removed_check(): \
        Command has been removed from favorites correctly...",
        Severity::Normal,
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::tools::testing_tools::TestCommand,
        utils::fs::{core::read_write::read_file_to_string, paths::PATHS},
    };

    #[test]
    fn command_name_is_removed_from_favorites_file() {
        let test_name = "command_name_is_removed_from_favorites_file";

        TestCommand::create(test_name, true);
        TestCommand::remove(test_name);

        let favorites_file_contents = read_file_to_string(&PATHS.favorites);

        assert!(!favorites_file_contents.contains(test_name));
    }
}
