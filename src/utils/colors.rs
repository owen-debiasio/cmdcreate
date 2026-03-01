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

use crate::{configs::load, utils::sys::args_contains};
use std::sync::LazyLock;

pub fn colors_enabled() -> bool {
    !args_contains("-m")
        && !args_contains("--monochrome")
        && !load("appearance", "disable_color", "false")
            .parse::<bool>()
            .unwrap()
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

pub static COLORS: LazyLock<Colors> = LazyLock::new(|| {
    if colors_enabled() {
        Colors {
            reset: "\x1b[0m",
            red: "\x1b[31m",
            green: "\x1b[32m",
            yellow: "\x1b[33m",
            blue: "\x1b[34m",
            magenta: "\x1b[35m",
            cyan: "\x1b[36m",
        }
    } else {
        Colors {
            reset: "",
            red: "",
            green: "",
            yellow: "",
            blue: "",
            magenta: "",
            cyan: "",
        }
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    fn get_all_codes(c: &Colors) -> Vec<&'static str> {
        vec![c.red, c.green, c.yellow, c.blue, c.reset]
    }

    #[test]
    fn colors_are_valid_ansi_codes_if_enabled() {
        let codes = get_all_codes(&*COLORS);

        if colors_enabled() {
            for color in codes {
                assert!(
                    color.starts_with("\x1b["),
                    "color does not start with ANSI escape: {color:?}"
                );
                assert!(
                    color.ends_with('m'),
                    "color does not end with 'm': {color:?}"
                );
            }
        } else {
            for color in codes {
                assert!(
                    color.is_empty(),
                    "color code should be empty in monochrome mode"
                );
            }
        }
    }

    #[test]
    fn reset_is_unique() {
        let c = &*COLORS;
        let codes = get_all_codes(&*COLORS);

        if colors_enabled() {
            for color in codes {
                if color != c.reset {
                    assert_ne!(color, c.reset, "reset color must be unique");
                }
            }
        }
    }
}
