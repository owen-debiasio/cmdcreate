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
    commands::tools::get_installed_commands,
    utils::{colors::COLORS, io::error},
};

pub fn search(command_to_search_for: &str) {
    let (yellow, blue, reset) = (COLORS.yellow, COLORS.blue, COLORS.reset);

    let mut command_search_index = 0;
    for command in get_installed_commands() {
        let command_stem = command.file_stem().unwrap_or_default().to_string_lossy();

        if command_stem.contains(command_to_search_for) {
            if command_search_index == 0 {
                println!("--------");
            }

            println!("{command_stem}");

            command_search_index += 1;
        }
    }

    if command_search_index == 0 {
        error(
            "No installed commands contain:",
            &format!("{yellow}\"{command_to_search_for}\"{reset}"),
        );
    }

    println!("--------");

    if command_search_index == 1 {
        println!("Found one match for {blue}\"{command_to_search_for}\"{reset}");
        return;
    }

    println!(
        "Found {blue}{command_search_index}{reset} matches for {blue}\"{command_to_search_for}\"{reset}."
    );
}
