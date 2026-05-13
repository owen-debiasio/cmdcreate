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
    commands::config::main::AVAILABLE_CATEGORIES,
    output,
    utils::{
        colors::COLORS,
        fs::{
            core::{read_file_to_string, write_to_file},
            paths::PATHS,
        },
        io::error,
    },
};

pub fn remove(category: &str, value: &str) {
    let green = COLORS.green;

    let config_path = PATHS.configuration_file;
    let config_file_contents = read_file_to_string(config_path);

    let category_header = AVAILABLE_CATEGORIES
        .iter()
        .find(|&&category_index| category_index.contains(category))
        .expect("Invalid category");

    let mut lines: Vec<String> = config_file_contents
        .lines()
        .map(ToString::to_string)
        .collect();

    let mut in_target_section = false;
    let mut removed_index: Option<usize> = None;

    for (line_index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed == *category_header {
            in_target_section = true;
            continue;
        }

        if in_target_section {
            if trimmed.starts_with('[') {
                break;
            }

            if trimmed.starts_with(value) && trimmed.contains('=') {
                removed_index = Some(line_index);
                break;
            }
        }
    }

    if let Some(line_position) = removed_index {
        lines.remove(line_position);

        let is_last_item = if line_position < lines.len() {
            lines[line_position].trim().starts_with('[')
        } else {
            true
        };

        if is_last_item && line_position > 0 && lines[line_position - 1].trim() == *category_header
        {
            lines.remove(line_position - 1);

            if line_position > 1 && lines[line_position - 2].trim().is_empty() {
                lines.remove(line_position - 2);
            }

            output!(
                "{green}Successfully removed config and empty category: {category}.",
                true
            );
        } else {
            output!(
                "{green}Successfully removed config: {value} from {category}.",
                true
            );
        }

        let new_contents = lines.join("\n");
        write_to_file(config_path, &new_contents, false);
    } else {
        error(
            &format!("Config key \"{value}\" not found in category:"),
            Some(category),
        );
    }
}
