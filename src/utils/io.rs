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

use std::process::exit;

use crate::{
    input,
    utils::{
        colors::COLORS,
        sys::arguments::{args_contains, arguments_force_actions},
    },
};

pub fn ask_for_confirmation(question: &str, exit_if_user_declines: bool) -> bool {
    let (red, green, reset) = (COLORS.red, COLORS.green, COLORS.reset);

    if arguments_force_actions() {
        return true;
    }

    let get_response_to_question = input!("{question}{reset}\n({green}Y{reset} or {red}N{reset})");

    if get_response_to_question.eq_ignore_ascii_case("y") {
        true
    } else if exit_if_user_declines {
        error("\n\nAborted.", None)
    } else {
        false
    }
}

pub fn output_is_silent() -> bool {
    args_contains("-s") || args_contains("--silent")
}

#[macro_export]
macro_rules! output {
    ($given_text:expr) => {
        $crate::output!($given_text, false);
    };

    ($given_text:expr, $include_arrow:expr) => {{
        let text_interpolated = format!($given_text);

        let text = if $crate::utils::io::output_is_silent() {
            ""
        } else {
            text_interpolated.trim()
        };

        let (blue, reset) = (
            $crate::utils::colors::COLORS.blue,
            $crate::utils::colors::COLORS.reset,
        );

        $crate::core::logger::main::log(text, $crate::core::logger::consts::Severity::Normal);

        println!(
            "{blue}{}{text}{reset}",
            if !text.is_empty() && $include_arrow {
                "> "
            } else {
                ""
            }
        );
    }};
}

#[macro_export]
macro_rules! input {
    ($given_text:expr) => {{
        use std::io::{self, Write};

        let (blue, reset) = (
            $crate::utils::colors::COLORS.blue,
            $crate::utils::colors::COLORS.reset,
        );

        let text_interpolated = format!($given_text);
        let text = text_interpolated.trim().replace("> ", "");

        // I'm using `println!` here to bypass the silent flag, because the user may need
        // to read the things that cmdcreate asks you.
        println!("{blue}> {text}{reset}");

        let _ = io::stdout().flush();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input.trim().to_string()
    }};
}

/// Error details are optional.
/// In that case, provide `None`
pub fn error(error_message: &str, error_details: Option<&str>) -> ! {
    let (red, reset) = (COLORS.red, COLORS.reset);

    if error_details.unwrap_or("").is_empty() {
        eprintln!("{red}Error: {}{reset}", error_message.trim());
    } else {
        eprintln!(
            "{red}Error: {} {}{reset}",
            error_message.trim(),
            error_details.unwrap()
        );
    }

    exit(1)
}
