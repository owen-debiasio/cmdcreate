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
    configs::load_configuration,
    utils::{
        colors::{COLORS, remove_spare_color_codes},
        sys::arguments::args_contains,
        fs::{paths::PATHS, core::write_to_file}
    },
};

#[derive(Copy, Clone)]
pub enum Severity {
    Normal = 0,
    Warn = 1,
}

// TODO: Make this a macro
pub fn log(text_to_log: &str, importance_level: Severity) {
    let (blue, cyan, yellow, reset) = (COLORS.blue, COLORS.cyan, COLORS.yellow, COLORS.reset);

    let time_format = &load_configuration("logs", "time_format", "%Y-%m-%d %H:%M:%S");
    let time = Local::now().format(time_format).to_string();

    let log_type = match importance_level {
        Severity::Warn => &format!("{yellow}WARN{reset}"),

        Severity::Normal => &format!("{cyan}LOG{reset}"),
    };

    // Should be located in /root/.local/share/cmdcreate/logs/
    let log_dir = &PATHS.log_directory;
    let log_file_name = &format!("{log_dir}/{time}.log");

    // Example:
    // [<time>] [ERROR] Uh oh this happened
    let log_file_text = format!("[{blue}{time}{reset}] [{log_type}] {text_to_log}\n");

    output_verbose_message(&log_file_text);

    // Remove things like "\x1b[35m" from being written to the log file. It looks stupid
    let finalized_output_text = remove_spare_color_codes(log_file_text);

    write_to_file(log_file_name, &finalized_output_text, true);
}

fn output_verbose_message(text_to_print: &str) {
    let verbose_flags_are_passed = args_contains("-V") || args_contains("--verbose");

    let verbose_enabled_in_config = load_configuration("logs", "verbose", "false")
        .parse::<bool>()
        .unwrap_or(false);

    if verbose_flags_are_passed || verbose_enabled_in_config {
        println!("{text_to_print}{}", COLORS.reset);
    }
}
