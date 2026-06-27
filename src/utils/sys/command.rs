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

pub fn system_command_is_installed(command_to_check: &str) -> bool {
    crate::run_shell_command!(bool: "which {command_to_check} > /dev/null")
}

#[macro_export]
macro_rules! run_shell_command {
    (bool: $($arg:tt)*) => {{
        use std::process::{Command, Stdio};

        let command_string = format!($($arg)*);
        let command = command_string.trim();

        if !command.is_empty() {
            let shell = $crate::core::configs::load::load_configuration("sys", "shell", "sh");

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

            match result {
                Ok(status) => status.success(),
                Err(error) => {
                    $crate::utils::io::error("Failed to launch shell:", Some(&error.to_string()));
                }
            }
        } else {
            true
        }
    }};

    ($($arg:tt)*) => {{
        let _ = $crate::run_shell_command!(bool: $($arg)*);
    }};
}
