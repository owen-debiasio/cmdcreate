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

use std::{env::args, sync::OnceLock};

pub fn args_contains(argument: &str) -> bool {
    static ARGS: OnceLock<Vec<String>> = OnceLock::new();

    let processed_args = ARGS.get_or_init(return_args);

    processed_args.iter().any(|arg| arg == argument)
}

pub fn return_args() -> Vec<String> {
    let mut supplied_argument_vector = Vec::new();

    let mut supplied_argument_is_actually_an_argument;

    for supplied_argument in args().skip(1) {
        supplied_argument_is_actually_an_argument = supplied_argument.starts_with('-');

        if supplied_argument_is_actually_an_argument && supplied_argument.len() > 2 {
            for character in supplied_argument.chars().skip(1) {
                supplied_argument_vector.push(format!("-{character}"));
            }
        } else {
            supplied_argument_vector.push(supplied_argument);
        }
    }

    supplied_argument_vector
}

pub fn arguments_force_actions() -> bool {
    args_contains("--force") || args_contains("-f")
}
