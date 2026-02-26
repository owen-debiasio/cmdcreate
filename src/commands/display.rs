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
    utils::{
        colors::COLORS,
        fs::{read_file_to_string, PATHS},
    },
};

pub fn display(cmd: &str) {
    command_is_installed(cmd);

    println!(
        "Contents of command: {}\"{cmd}\"{}\n--------\n{}",
        COLORS.blue,
        COLORS.reset,
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir))
            .expect("Failed to retrieve command contents")
            .trim() // Remove extra whitespace just in case
    );
}
