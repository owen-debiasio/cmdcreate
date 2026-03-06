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

use chrono::Local;

use crate::{
    configs::load,
    utils::{
        colors::COLORS,
        fs::{PATHS, create_folder, write_to_file},
        sys::args_contains,
    },
};

pub fn log(text: &str, lvl: u8) {
    let time = Local::now()
        .format(&load("logs", "time_format", "%Y-%m-%d %H:%M:%S"))
        .to_string();

    let (color, log_type) = match lvl {
        0 => (COLORS.cyan, "LOG"),
        1 => (COLORS.yellow, "WARN"),
        2 => (COLORS.red, "ERROR"), // Like this is ever used. It's used once in utils/io::error()
        _ => (COLORS.reset, "LOG"),
    };

    let (log_text, log_dir) = (format!("[{log_type}] {text}"), &PATHS.log_dir);

    if args_contains("-V")
        || args_contains("--verbose")
        || load("logs", "verbose", "false").parse::<bool>().unwrap()
    {
        println!("{color}{time} {log_text}{}", COLORS.reset);
    }

    create_folder(log_dir).expect("Failed to create folder");

    write_to_file(
        &format!("{log_dir}/{time}.log"),
        &format!("{time} {log_text}\n"),
        true,
    )
    .expect("Failed to write to file");
}
