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

#[macro_export]
macro_rules! validate_args {
    ($command:expr, $amount_of_arguments_given:expr, $amount_of_arguments_needed:expr, $command_usage:expr, $additional_args:expr) => {
        if $amount_of_arguments_given.len() <= $amount_of_arguments_needed {
            use $crate::utils::colors::COLORS;

            let (blue, yellow, red, green, reset) = (
                COLORS.blue,
                COLORS.yellow,
                COLORS.red,
                COLORS.green,
                COLORS.reset,
            );

            let include_additional_flags = if !$additional_args.is_empty() {
                &format!("{reset}[{green}{}{reset}] ", $additional_args)
            } else {
                ""
            };

            println!(
                "Usage:\ncmdcreate {blue}{} {include_additional_flags}{yellow}{}{red}{reset}",
                $command, $command_usage
            );

            return;
        }
    };
}
