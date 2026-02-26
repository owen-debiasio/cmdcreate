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

use std::{fs::read_dir, path::PathBuf};

use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::error,
    },
};

pub fn command_is_installed(cmd: &str) -> bool {
    if path_exists(&format!("{}{cmd}", PATHS.install_dir)) {
        log(
            &format!(
                "commands/tools::command_is_installed(): Command \"{cmd}\" is installed... Continuing..."
            ),
            0,
        );

        return true;
    }

    error(&format!("Command \"{cmd}\" is not installed"), "");
}

pub fn get_installed_commands() -> Vec<PathBuf> {
    let (red, reset) = (COLORS.red, COLORS.reset);

    log(
        "commands/tools::get_installed_commands(): Getting installed commands...",
        0,
    );

    let commands: Vec<PathBuf> = read_dir(&PATHS.install_dir)
        .unwrap_or_else(|_| panic!("{red}Error: Failed to read install directory{reset}",))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if commands.is_empty() {
        error("No commands are installed.", "");
    }

    commands
}
