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

use crate::commands::core::edit::get_available_editor;
use crate::{
    commands::{
        config::{config_add::add, config_remove::remove},
        doc::{doc, view_documentation_file},
    },
    run_shell_command,
    utils::{
        fs::{misc::use_pager_on_file, paths::PATHS},
        io::error,
        sys::env::root_check,
    },
};

pub static AVAILABLE_CATEGORIES: &[&str] = &["[appearance]", "[logs]", "[sys]"];
pub static AVAILABLE_VALUES: &[&str] = &[
    "shell",
    "time_format",
    "verbose",
    "favorite_indicator",
    "disable_color",
];

fn edit_config_file() {
    let editor = get_available_editor();
    let config_file = PATHS.configuration_file;

    run_shell_command!("{editor} {config_file}");
}

pub fn config(mode: &str, category: &str, value: &str) {
    // I have to make a whole separate match statement for this bro
    match mode {
        "add" | "remove" | "edit" => root_check(),
        _ => (), // Continue if none of those
    }

    match mode {
        "add" | "remove" => init_config_changes(mode, category, value),

        "help" => doc("configurations"),
        "example" => view_documentation_file("docs/resources/config_example.toml"),

        "edit" => edit_config_file(),
        "display" => use_pager_on_file(PATHS.configuration_file),

        _ => error("Invalid action:", Some(mode)),
    }
}

fn init_config_changes(config_mode: &str, config_category: &str, config_value: &str) {
    let category_header = format!("[{config_category}]");

    if config_category.is_empty() {
        error("Please provide a category.", None)
    } else if !AVAILABLE_CATEGORIES.contains(&category_header.as_str()) {
        error("Not a valid category:", Some(config_category));
    }

    if config_value.is_empty() {
        error("Please provide a value.", None)
    } else if !AVAILABLE_VALUES.contains(&config_value) {
        error("Not a valid value:", Some(config_value));
    }

    if config_mode == "add" {
        add(config_category, config_value);
    } else if config_mode == "remove" {
        remove(config_category, config_value);
    }
}
