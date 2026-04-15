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

use std::{
    fmt::{Display, Formatter},
    io::stdin,
    process::exit,
};

use crate::utils::{colors::COLORS, sys::arguments::arguments_force_actions};

pub fn ask_for_confirmation(question: &str, exit_if_user_declines: bool) -> bool {
    let (red, green, reset) = (COLORS.red, COLORS.green, COLORS.reset);

    if arguments_force_actions() {
        return true;
    }

    let question_that_is_asked = &format!("{question}{reset}\n({green}Y{reset} or {red}N{reset})");
    let get_response_to_question = input(question_that_is_asked);

    if get_response_to_question.eq_ignore_ascii_case("y") {
        true
    } else if exit_if_user_declines {
        error("\n\nAborted.", "")
    } else {
        false
    }
}

pub fn input(text: &str) -> String {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    println!("{blue}{text}{reset}");

    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();

    user_input.trim().to_string()
}

/// Error details are optional.
/// In that case, provide the string but leave it empty
pub fn error(error_message: &str, error_details: &str) -> ! {
    let (red, reset) = (COLORS.red, COLORS.reset);

    eprintln!(
        "{red}Error: {} {error_details}{reset}",
        error_message.trim()
    );

    exit(1)
}

#[derive(Debug)]
pub struct TestError(pub String);

use core::fmt;

impl Display for TestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[allow(dead_code)]
pub fn error_result<T>(error_result_message: &str) -> Result<T, TestError> {
    Err(TestError(error_result_message.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_returns_err() {
        let error_result: Result<(), _> = error_result("nope");
        assert!(error_result.is_err());
    }

    #[test]
    fn error_message_matches() {
        assert_eq!(error_result::<()>("bad").unwrap_err().to_string(), "bad");
    }
}
