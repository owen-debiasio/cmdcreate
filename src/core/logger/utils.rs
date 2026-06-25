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
    core::configs::load::load_configuration,
    utils::fs::{core::read_write::write_to_file, paths::PATHS},
};
use chrono::Local;

pub fn get_formatted_timestamp() -> String {
    let time_format = load_configuration("logs", "time_format", "%Y-%m-%d %H:%M:%S");
    Local::now().format(&time_format).to_string()
}

pub fn save_log_to_file(timestamp: &str, plain_text: &str) {
    let log_dir = &PATHS.log_directory;
    let log_file_name = format!("{log_dir}/{timestamp}.log");
    write_to_file(&log_file_name, plain_text, true);
}
