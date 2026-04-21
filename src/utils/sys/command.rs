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

use std::process::Command;

pub fn system_command_is_installed(command_to_check: &str) -> bool {
    Command::new("which")
        .arg(command_to_check)
        .output()
        .is_ok_and(|output_status| output_status.status.success())
}

#[macro_export]
macro_rules! run_shell_command {
    ($given_command:expr) => {{
        use std::process::{Command, Stdio};

        let command_interpolated = format!($given_command);
        let command = command_interpolated.trim();

        if !command.is_empty() {
            let shell = $crate::configs::load_configuration("sys", "shell", "sh");

            match Command::new(shell)
                .arg("-c")
                .arg(command)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
            {
                Ok(_) => {}
                Err(e) => $crate::utils::io::error("Failed to run command:", &e.to_string()),
            }
        }
    }};
}
