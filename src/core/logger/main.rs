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

use crate::{
    core::logger::{
        consts::Severity,
        utils::{get_formatted_timestamp, save_log_to_file},
        verbose::output_verbose_message,
    },
    utils::colors::{COLORS, remove_spare_color_codes},
};

pub fn log(text_to_log: &str, importance_level: Severity) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let time = get_formatted_timestamp();
    let log_type = importance_level.to_colored_string();

    // Example output:
    // [<date> <time>] [<message type>] path/to/file::function(): Message
    let console_output = format!("[{blue}{time}{reset}] [{log_type}] {text_to_log}\n");
    output_verbose_message(&console_output);

    let file_output = remove_spare_color_codes(console_output);
    save_log_to_file(&time, &file_output);
}
