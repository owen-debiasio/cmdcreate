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
        config::{
            actions::{add::add, remove::remove},
            list::list,
            verify::verify_setting,
        },
        doc::{doc, view_documentation_file},
    },
    utils::{
        fs::{core::read_write::edit_file_in_text_editor, misc::use_pager_on_file, paths::PATHS},
        io::{ask_for_confirmation, error},
        sys::env::running_as_root,
    },
};

pub static AVAILABLE_SETTINGS: &[&str] = &[
    "[self]:disable_root_usage",
    "[sys]:shell",
    "[logs]:time_format,verbose",
    "[appearance]:favorite_indicator,disable_color",
    "[internet]:force_disable,sample_dns",
    "[update]:zig_version",
];

/// How the different values are organized:
/// `mode`: add, remove, etc.
/// `category`: self, appearance, logs, sys, etc.
/// `key_with_value`: key="value"
pub fn config(mode: &str, category: &str, key_with_value: &str) {
    match mode {
        "add" | "remove" => init_config_changes(mode, category, key_with_value),

        "help" => doc("configurations"),
        "example" => view_documentation_file("docs/resources/config_example.toml"),

        "edit" => edit_file_in_text_editor(PATHS.configuration_file),
        "display" => use_pager_on_file(PATHS.configuration_file),
        "list" => list(),
        _ => error("Invalid action:", Some(mode)),
    }
}

fn init_config_changes(mode: &str, category: &str, key_with_value: &str) {
    let key = key_with_value.split('=').next().unwrap();
    let value = key_with_value.split('=').next_back().unwrap();
    let category_header = format!("[{category}]");

    // Manual override to prevent easy exploiting and security reasons. I don't know
    // where else to put this code tbh
    if key == "disable_root_usage" {
        if running_as_root() {
            if mode == "add" {
                ask_for_confirmation(
                    "Are you sure that you want to enable this setting? \
                    You may not be able to revert this.",
                    true,
                );
            }
        } else {
            if mode == "remove" {
                error(
                    "You can't re-enable root when running as a normal user.",
                    Some("Please see: 'cat /etc/cmdcreate.toml'."),
                )
            }
            if mode == "add" {
                error(
                    "You can only enable this setting with root privileges.",
                    None,
                )
            }
        }
    }

    verify_setting(mode, &category_header, key, value);

    if mode == "add" {
        add(category, key, value);
    } else if mode == "remove" {
        remove(category, key_with_value);
    }
}
