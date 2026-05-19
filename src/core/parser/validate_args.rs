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

use crate::utils::colors::COLORS;

#[macro_export]
macro_rules! validate_args {
    ($command:expr,
    $amount_of_arguments_given:expr,
    $amount_of_arguments_needed:expr,
    $subcommands_with_desc:expr, // Formatted like this: "&command:desc+command:desc+..."
    $additional_args:expr
    ) => {
        if $amount_of_arguments_given.len() <= $amount_of_arguments_needed {
            use $crate::utils::colors::COLORS;

            let (blue, yellow, magenta, green, reset) = (
                COLORS.blue,
                COLORS.yellow,
                COLORS.magenta,
                COLORS.green,
                COLORS.reset,
            );

            let include_additional_flags = if !$additional_args.is_empty() {
                &format!("{reset}[{green}{}{reset}] ", $additional_args)
            } else {
                ""
            };

            let formatted_subcommand_list =
                $crate::core::parser::validate_args::format_subcommand_list(stringify!($subcommands_with_desc));

            println!(
                "{blue}> Usage:\n{reset}cmdcreate {yellow}{} {blue}[commands/fields] {magenta}{include_additional_flags}{reset}\n\
                Commands/Fields:\n\
                {formatted_subcommand_list}",
                $command
            );

            return;
        }
    };
}

pub fn format_subcommand_list(list: &str) -> String {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let cleaned_string = list.replace(['+', '"'], "").replace('&', blue);

    let lines: Vec<&str> = cleaned_string.lines().collect();

    let max_cmd_len = lines
        .iter()
        .filter_map(|line| line.split_once(':'))
        .map(|(command, _)| command.trim().len())
        .max()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            if let Some((command, desc)) = line.split_once(':') {
                let trimmed_command = command.trim();
                let trimmed_desc = desc.trim();
                format!("{trimmed_command:max_cmd_len$} {reset}: {trimmed_desc}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
        .replace(':', " ")
}
