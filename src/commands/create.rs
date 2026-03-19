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
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, overwrite_file},
        io::error,
        sys::run_shell_command,
    },
};

// Uniquely identify commands created by cmdcreate from
// commands that may also show up in /usr/local/bin/
pub static NEW_COMMAND_HEADER: &str = "# Created by cmdcreate\n";

pub fn create(
    command_to_create: &str,
    contents_of_new_command_by_user: &str,
    run_this_function_verbose: bool,
) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "commands/create::create(): Creating command \"{command_to_create}\": \
            With contents \"{contents_of_new_command_by_user}\"{}",
            if run_this_function_verbose {
                ", and being verbose..."
            } else {
                "..."
            }
        ),
        0,
    );

    let full_contents_of_new_command =
        &format!("{NEW_COMMAND_HEADER}{contents_of_new_command_by_user}");

    let path_to_command = &format!(
        "{}{command_to_create}",
        PATHS.command_installation_directory
    );

    log(
        &format!("commands/create::create(): Command path: \"{path_to_command}\""),
        0,
    );

    if contents_of_new_command_by_user.is_empty() {
        error("The contents of your command can not be empty.", "");
    }

    log(
        &format!("commands/create::create(): Writing contents to script: \"{path_to_command}\""),
        0,
    );

    overwrite_file(path_to_command, full_contents_of_new_command).expect("Failed to write to file");

    log("commands/create::create(): Activating command...", 0);

    run_shell_command(&format!("chmod +x {path_to_command}"));

    if run_this_function_verbose {
        println!("\n{green}Success! Created command: {blue}\"{command_to_create}\"{reset}");
    }
}
