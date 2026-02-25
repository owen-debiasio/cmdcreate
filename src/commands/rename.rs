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
        remove::remove,
        tools::{command_is_installed, get_installed_commands},
    },
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::{error, input},
        sys::{args_forced, run_shell_command},
    },
};

pub fn rename(old: &str, new: &str) {
    let (blue, red, yellow, green, reset) = (
        COLORS.blue,
        COLORS.red,
        COLORS.yellow,
        COLORS.green,
        COLORS.reset,
    );

    let (install_dir, installed_commands) = (&PATHS.install_dir, get_installed_commands());

    if installed_commands.is_empty() {
        return;
    }

    command_is_installed(old);

    if path_exists(&format!("{}/{new}", PATHS.install_dir)) {
        println!(
            "{red}The new name ({yellow}{new}{red}) is already installed! Do you want to delete it?\n({green}Y{red} or {yellow}N{red})",
        );

        if args_forced() || input("").trim().eq_ignore_ascii_case("y") {
            remove(new, true);
        } else {
            error("You need to remove the old command before proceeding!", "");
        }
    }

    log(
        &format!("commands/rename::rename(): Renaming command \"{old}\" to \"{new}\"..."),
        0,
    );

    run_shell_command(&format!("mv {install_dir}{old} {install_dir}{new}"));

    println!("{green}Successfully renamed command {blue}\"{old}\" to {blue}\"{new}\"{reset}");
}
