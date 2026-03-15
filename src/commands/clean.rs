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

use crate::utils::{
    colors::COLORS,
    fs::{PATHS, delete_folder},
    io::{ask_for_confirmation, input},
    sys::arguments_force_actions,
};

pub fn clean() {
    let (green, red, reset) = (COLORS.green, COLORS.red, COLORS.reset);

    ask_for_confirmation("Do you want to clean cmdcreate?");

    let question = input(&format!(
        "\nDo you want to delete old log files?{reset}\n({green}Y{reset} or {red}N{reset})"
    ));
    if !arguments_force_actions() && question.trim().eq_ignore_ascii_case("y") {
        delete_folder(&PATHS.log_directory).expect("Failed to delete log directory");

        println!("{green}\nLog files cleared.{reset}");
    }

    println!("{green}\nCleaned up cmdcreate.{reset}");
}
