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
    commands::{favorite::favorite, tools::command_is_installed},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{delete_file, read_file_to_string, PATHS},
        io::ask_for_confirmation,
    },
};

pub fn remove(command: &str, forced: bool) {
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    if command.is_empty() {
        log("commands/remove::remove(): Command is empty, exiting...", 0);
        return;
    }

    command_is_installed(command);

    if !forced {
        ask_for_confirmation(&format!(
            "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
        ));
    }

    if read_file_to_string(&PATHS.favorites)
        .expect("Failed to retrieve favorites")
        .contains(command)
    {
        favorite("remove", command);
    }

    log(
        &format!("commands/remove::remove(): Removing command \"{command}\"..."),
        0,
    );

    delete_file(&format!("{}{command}", PATHS.install_dir)).expect("Failed to delete command");

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
