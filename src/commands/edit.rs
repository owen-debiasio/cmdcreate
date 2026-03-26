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

use crate::logger::Severity;
use crate::{
    commands::tools::determine_command_is_installed,
    logger::log,
    utils::{
        fs::PATHS,
        io::error,
        sys::{ENVIRONMENT_VARIABLES, run_shell_command},
    },
};
use std::process::Command;

pub fn edit(command_to_edit: &str) {
    determine_command_is_installed(command_to_edit);

    let available_editors: &[&str] = &[
        &ENVIRONMENT_VARIABLES.text_editor, // Used when user runs something like "EDITOR=vi cmdcreate edit abc"
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

    let chosen_editor = available_editors.iter().find(|&&editor_to_find| {
        Command::new("which")
            .arg(editor_to_find)
            .output()
            .map(|output_status| output_status.status.success())
            .unwrap_or(false)
    });

    let editor = chosen_editor.map_or_else(
        || {
            error("No known editor is installed on your device.", "");
        },
        |editor_that_is_chosen| *editor_that_is_chosen,
    );

    log(
        &format!("commands/edit::edit(): Using editor \"{editor}\"..."),
        Severity::Normal,
    );

    run_shell_command(&format!(
        "{editor} {}{command_to_edit}",
        PATHS.command_installation_directory
    ));
}
