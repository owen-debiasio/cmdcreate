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

use cmdcreate::{
    core::{
        init::init,
        logger::{consts::Severity, main::log},
        parser::{parse::parse, usage::cmdcreate_usage},
    },
    utils::sys::arguments::return_args,
};

use std::process::exit;

fn main() {
    init();

    log("main::main(): Retrieving args...", Severity::Normal);

    let mut arguments_retrieved = return_args();

    // These flags or args or whatever you call them are basically "ignored"
    arguments_retrieved.retain(|argument| {
        !matches!(
            argument.as_str(),
            "-V" | "--verbose" | "-o" | "--offline" | "-m" | "--monochrome" | "-s" | "--silent"
        )
    });

    if arguments_retrieved.is_empty() {
        cmdcreate_usage();
    }

    let arguments_to_parse = arguments_retrieved.iter().enumerate();

    for (index_of_provided_arguments, command) in arguments_to_parse {
        if !command.starts_with('-') {
            parse(command, &arguments_retrieved[index_of_provided_arguments..]);
            return;
        }

        parse(command, &arguments_retrieved);
    }

    exit(0)
}
