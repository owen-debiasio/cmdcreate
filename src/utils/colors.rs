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

use crate::{configs::load_configuration, utils::sys::args_contains};
use std::sync::LazyLock;

pub fn colors_enabled() -> bool {
    let colors_are_enabled_via_flags = !args_contains("-m") && !args_contains("--monochrome");

    let colors_are_enabled_via_set_configs =
        !load_configuration("appearance", "disable_color", "false")
            .parse::<bool>()
            .unwrap_or(false);

    colors_are_enabled_via_flags && colors_are_enabled_via_set_configs
}

pub struct Colors {
    pub reset: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
}

impl Colors {
    pub const fn new(colors_are_enabled: bool) -> Self {
        if colors_are_enabled {
            Self {
                reset: "\x1b[0m",
                red: "\x1b[31m",
                green: "\x1b[32m",
                yellow: "\x1b[33m",
                blue: "\x1b[34m",
                magenta: "\x1b[35m",
                cyan: "\x1b[36m",
            }
        } else {
            Self {
                reset: "",
                red: "",
                green: "",
                yellow: "",
                blue: "",
                magenta: "",
                cyan: "",
            }
        }
    }
}

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::new(colors_enabled()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_valid_ansi_codes_when_enabled() {
        let color = Colors::new(true);
        let color_codes = vec![
            color.red,
            color.green,
            color.yellow,
            color.blue,
            color.magenta,
            color.cyan,
            color.reset,
        ];

        for code in color_codes {
            assert!(code.starts_with("\x1b["));
            assert!(code.ends_with('m'));
        }
    }

    #[test]
    fn colors_are_empty_when_disabled() {
        let color = Colors::new(false);
        let color_codes = vec![
            color.red,
            color.green,
            color.yellow,
            color.blue,
            color.magenta,
            color.cyan,
            color.reset,
        ];

        for code in color_codes {
            assert!(code.is_empty());
        }
    }

    #[test]
    fn reset_is_unique_when_enabled() {
        let colors = Colors::new(true);
        let color_codes = vec![
            colors.red,
            colors.green,
            colors.yellow,
            colors.blue,
            colors.magenta,
            colors.cyan,
        ];

        for code in color_codes {
            assert_ne!(code, colors.reset);
        }
    }
}
