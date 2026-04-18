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
    commands::{create::NEW_COMMAND_HEADER, tools::cmdcreate_command_is_installed},
    output,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(command_to_display: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    cmdcreate_command_is_installed(command_to_display);

    let path_to_command = format!(
        "{}{command_to_display}",
        PATHS.command_installation_directory
    );

    // Remove command header because you
    // already know cmdcreate made the command
    let contents_of_command = read_file_to_string(&path_to_command).replace(NEW_COMMAND_HEADER, "");
    let trimmed_contents_of_command = contents_of_command.trim();

    output!(
        "Contents of command: \
        {blue}\"{command_to_display}\"{reset}\n\
        --------\n\
        {trimmed_contents_of_command}"
    );
}
