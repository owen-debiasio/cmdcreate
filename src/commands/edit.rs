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
    commands::tools::command_is_installed,
    logger::log,
    utils::{
        fs::PATHS,
        io::error,
        sys::{run_shell_command, VARS},
    },
};
use std::process::Command;

pub fn edit(cmd: &str) {
    command_is_installed(cmd);

    let editors: &[&str] = &[
        &VARS.editor, // Used when user runs something like "EDITOR=vi cmdcreate edit abc"
        "nvim",
        "vi",
        "vim",
        "nano",
        "micro",
        "code",
        "code-insiders",
        "gedit",
        "kate",
        "kwrite",
        "emacs",
        "vscodium",
        "vscodium-insiders",
        "zed",
        "zed-preview",
        "mousepad",
    ];

    let editor = editors.iter().find(|&&ed| {
        Command::new("which")
            .arg(ed)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    });

    let editor = editor.map_or_else(
        || {
            error("No known editor is installed on your device.", "");
        },
        |ed| *ed,
    );

    log(
        &format!("commands/edit::edit(): Using editor \"{editor}\"..."),
        0,
    );

    run_shell_command(&format!("sudo {editor} {}{cmd}", PATHS.install_dir));
}
