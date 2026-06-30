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
    core::logger::{consts::Severity, main::log},
    utils::io::error,
};
use rustix::process::geteuid;
use std::{env::var, sync::LazyLock};

pub fn running_as_root() -> bool {
    geteuid().as_raw() == 0
}

pub fn root_check() {
    log(
        "utils::sys::env::root_check(): \
        Checking root status...",
        Severity::Normal,
    );

    if !running_as_root() {
        error(
            "\
        To execute this action, \
        please run cmdcreate as root.",
            None,
        )
    }
}

pub struct Vars {
    pub shell: String,
    pub text_editor: String,
    pub home: String,
}

pub static ENVIRONMENT_VARIABLES: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_owned()),
    text_editor: var("EDITOR").unwrap_or_else(|_| "auto".to_owned()),
    home: var("HOME").unwrap_or_else(|_| "auto".to_owned()),
});

#[test]
fn vars_are_initialized() {
    assert!(!ENVIRONMENT_VARIABLES.shell.is_empty());
    assert!(!ENVIRONMENT_VARIABLES.text_editor.is_empty());
    assert!(!ENVIRONMENT_VARIABLES.home.is_empty());
}
