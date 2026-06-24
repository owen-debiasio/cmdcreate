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
    core::logger::{consts::Severity, main::log},
    output,
    utils::{
        fs::{
            core::{read_file_to_string, write_to_file},
            paths::PATHS,
        },
        io::error,
        sys::env::running_as_root,
    },
};

pub fn add(category: &str, full_setting: &str) {
    let config_path = PATHS.configuration_file;
    let config_file_contents = read_file_to_string(config_path);

    log(
        &format!(
            "commands/config/config_add::add(): \
        Config full setting: {full_setting}"
        ),
        Severity::Normal,
    );

    let value = full_setting.split('=').next_back().unwrap();
    let key = full_setting.split('=').next().unwrap();

    log(
        &format!(
            "commands/config/config_add::add(): \
        Config value: {value}"
        ),
        Severity::Normal,
    );
    log(
        &format!(
            "commands/config/config_add::add(): \
        Config key: {key}"
        ),
        Severity::Normal,
    );

    if key == "disable_root_usage" && !running_as_root() {
        error(
            "You can only enable this setting with root privileges.",
            None,
        )
    }

    if key.is_empty() || value.is_empty() || !full_setting.contains('=') {
        error("Please provide a setting.", None)
    }

    let parts: Vec<&str> = full_setting.splitn(2, '=').collect();
    let key = parts[0];
    let mut setting = parts[1].to_string();

    if !setting.starts_with('"') && !setting.ends_with('"') {
        setting = format!("\"{setting}\"");
    }
    let sanitized_value = format!("{key}={setting}");

    let category_header = AVAILABLE_CATEGORIES
        .iter()
        .find(|&&category_index| category_index.contains(category))
        .expect("Invalid category");

    let mut lines: Vec<String> = config_file_contents
        .lines()
        .map(ToString::to_string)
        .collect();

    let (mut in_target_section, mut replaced) = (false, false);

    let mut section_end_index = lines.len();

    for (line_index, line) in lines.iter_mut().enumerate() {
        let trimmed = line.trim();

        if trimmed == *category_header {
            in_target_section = true;
            continue;
        }

        if in_target_section {
            if trimmed.starts_with('[') {
                section_end_index = line_index;
            }

            if trimmed.starts_with(key) && trimmed.contains('=') {
                line.clone_from(&sanitized_value);
                replaced = true;
            }

            break;
        }
    }

    if replaced {
    } else if in_target_section {
        lines.insert(section_end_index, sanitized_value);
    } else {
        lines.push(format!("\n{category_header}"));
        lines.push(sanitized_value);
    }

    let new_contents = lines.join("\n");
    write_to_file(config_path, &new_contents, false);

    output!("Successfully updated config: {key} set to {setting}.", true);
}
