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
    configs::load,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn list() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let installed_scripts = get_installed_commands();

    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_scripts.len()
    );

    for script in installed_scripts {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();
        let favorites =
            read_file_to_string(&PATHS.favorites).expect("Failed to retrieve favorites");

        log(
            &format!(
                "commands/list::list(): Current command: \"{name}\" (favorited={})",
                favorites.contains(name.to_string().as_str())
            ),
            0,
        );

        if favorites.contains(name.to_string().as_str()) {
            println!(
                "{} {name}",
                load("appearance", "favorite_indicator", "\u{2605}")
            );

            continue;
        }

        if favorites.is_empty() {
            println!("{name}");
            continue;
        }

        println!("  {name}");
    }
}
