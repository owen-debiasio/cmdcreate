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
    commands::tools::cmdcreate_command_is_installed,
    logger::{Severity, log},
    utils::{
        fs::PATHS,
        io::error,
        sys::{
            command::{run_shell_command, system_command_is_installed},
            env::ENVIRONMENT_VARIABLES,
        },
    },
};

pub fn get_available_editor() -> String {
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

    let chosen_editor = available_editors
        .iter()
        .find(|&&editor_to_find| system_command_is_installed(editor_to_find));

    chosen_editor
        .map_or_else(
            || {
                error("No known editor is installed on your device.", "");
            },
            |editor_that_is_chosen| *editor_that_is_chosen,
        )
        .to_string()
}

pub fn edit(command_to_edit: &str) {
    cmdcreate_command_is_installed(command_to_edit);

    let editor_to_use = get_available_editor();

    log(
        &format!("commands/edit::edit(): Using editor \"{editor_to_use}\"..."),
        Severity::Normal,
    );

    let command_to_edit_command = &format!(
        "{editor_to_use} {}{command_to_edit}",
        PATHS.command_installation_directory
    );

    run_shell_command(command_to_edit_command);
}
