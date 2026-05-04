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
    ($($arg:tt)*) => {{
        use std::process::{Command, Stdio};

        let command_string = format!($($arg)*);
        let command = command_string.trim();

        if !command.is_empty() {
            let shell = $crate::configs::load_configuration("sys", "shell", "sh");

            // Determine if terminal output is visible
            let stdout = if $crate::utils::io::output_is_silent() {
                Stdio::null()
            } else {
                Stdio::inherit()
            };

            let result = Command::new(shell)
                .arg("-c")
                .arg(command)
                .stdin(Stdio::inherit())
                .stdout(stdout)
                .stderr(Stdio::inherit())
                .status();

            if let Err(e) = result {
                $crate::utils::io::error("Failed to launch shell:", &e.to_string());
            }
        }
    }};
}
