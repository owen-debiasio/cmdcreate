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
    utils::{colors::COLORS, io::output_is_silent, sys::arguments::args_contains},
};

pub fn is_verbose_enabled() -> bool {
    let verbose_args_passed = args_contains("-V") || args_contains("--verbose");

    let verbose_enabled_via_config = load_configuration("logs", "verbose", "false")
        .parse::<bool>()
        .unwrap_or(false);

    verbose_args_passed || verbose_enabled_via_config
}

pub fn output_verbose_message(text_to_print: &str) {
    if is_verbose_enabled() && !output_is_silent() {
        // Using print! because text_to_log already ends with \n
        print!("{text_to_print}{}", COLORS.reset);
    }
}
