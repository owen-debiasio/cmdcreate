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

use crate::{commands::config::main::AVAILABLE_SETTINGS, utils::io::error};

pub fn verify_setting(mode: &str, category: &str, key: &str, value: &str) -> bool {
    if category.replace(['[', ']'], "").is_empty() {
        error("Please provide a category.", None)
    } else if !AVAILABLE_SETTINGS
        .iter()
        .any(|settings| settings.starts_with(category))
    {
        error("Not a valid category:", Some(category))
    }

    let list_index = AVAILABLE_SETTINGS
        .iter()
        .position(|&settings| settings.contains(category));

    let available_keys = AVAILABLE_SETTINGS[list_index.unwrap()]
        .split(':')
        .next_back()
        .unwrap();

    if key == value && mode != "remove" {
        error("Please provide a setting.", None)
    }

    if !available_keys.contains(key) {
        error("Not a valid key:", Some(key));
    }

    if value.is_empty() {
        error("Please provide a value.", None)
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::commands::config::verify::verify_setting;

    #[test]
    fn correct_setting_formatting_passes() {
        assert!(verify_setting("add", "[sys]", "shell", "bash"));
    }
}
