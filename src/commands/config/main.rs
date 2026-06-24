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
    commands::{
        config::{config_add::add, config_remove::remove},
        core::edit::get_available_editor,
        doc::{doc, view_documentation_file},
    },
    run_shell_command,
    utils::{
        fs::{misc::use_pager_on_file, paths::PATHS},
        io::error,
        sys::env::running_as_root,
    },
};

pub static AVAILABLE_CATEGORIES: &[&str] =
    &["[self]", "[appearance]", "[logs]", "[sys]", "[internet]"];
pub static AVAILABLE_VALUES: &[&str] = &[
    "shell",
    "time_format",
    "verbose",
    "favorite_indicator",
    "disable_color",
    "force_disable",
    "sample_dns",
    "disable_root_usage",
];

fn edit_config_file() {
    let editor = get_available_editor();
    let config_file = PATHS.configuration_file;

    run_shell_command!("{editor} {config_file}");
}

pub fn config(mode: &str, category: &str, key: &str) {
    // Manual override to prevent easy exploiting and security reasons. I don't know
    // where else to put this code tbh
    if key == "disable_root_usage" && mode == "remove" && !running_as_root() {
        error(
            "You can't re-enable root when running as a normal user.",
            Some("Try running: 'cat /etc/cmdcreate.toml'."),
        )
    }

    match mode {
        "add" | "remove" => init_config_changes(mode, category, key),

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

    let config_value_raw = config_value.split('=').next().unwrap();

    if config_value.is_empty() {
        error("Please provide a value.", None)
    } else if !AVAILABLE_VALUES.contains(&config_value_raw) {
        error("Not a valid value:", Some(config_value_raw));
    }

    if config_mode == "add" {
        add(config_category, config_value);
    } else if config_mode == "remove" {
        remove(config_category, config_value);
    }
}
