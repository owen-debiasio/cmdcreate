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

use crate::utils::fs::{core::read_file_to_string, paths::PATHS};

use std::sync::LazyLock;
use toml::{Value, from_str, map::Map};

static CONFIG: LazyLock<Value> = LazyLock::new(|| {
    let config_file_contents = read_file_to_string(PATHS.configuration_file);

    from_str(&config_file_contents).unwrap_or_else(|_| Value::Table(Map::new()))
});

pub fn load_configuration(
    config_category: &str,
    config_value: &str,
    default_value: &str,
) -> String {
    CONFIG
        .get(config_category)
        .and_then(|category| category.get(config_value))
        .and_then(|value| value.as_str())
        .unwrap_or(default_value)
        .to_string()
}
