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
    configs::load_configuration,
    utils::{io::error, sys::env::ENVIRONMENT_VARIABLES},
};

use std::process::{Command, Stdio};

pub fn system_command_is_installed(command_to_check: &str) -> bool {
    Command::new("which")
        .arg(command_to_check)
        .output()
        .map(|output_status| output_status.status.success())
        .unwrap_or(false)
}

pub fn run_shell_command(command: &str) {
    let shell: &str = &load_configuration("sys", "shell", &ENVIRONMENT_VARIABLES.shell);

    if command.trim().is_empty() {
        return;
    }

    match Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {}
        Err(error_message) => {
            error("Failed to run shell command:", &error_message.to_string());
        }
    }
}
