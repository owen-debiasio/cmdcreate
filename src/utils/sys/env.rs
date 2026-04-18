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

use rustix::process::geteuid;
use std::{env::var, sync::LazyLock};

use crate::utils::sys::arguments::args_contains;

pub fn running_as_root() -> bool {
    // 0 is root, if it returns anything else, cmdcreate won't run
    geteuid().as_raw() == 0
}

pub fn root_requirement_is_bypassed() -> bool {
    // Either of these flags allow root bypass
    args_contains("-b") || args_contains("--bypass-root")
}

pub struct Vars {
    pub shell: String,
    pub text_editor: String,
}

pub static ENVIRONMENT_VARIABLES: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_owned()),
    text_editor: var("EDITOR").unwrap_or_else(|_| "auto".to_owned()),
});

#[cfg(test)]
mod tests {
    use crate::utils::sys::env::ENVIRONMENT_VARIABLES;

    #[test]
    fn vars_are_initialized() {
        assert!(!ENVIRONMENT_VARIABLES.shell.is_empty());
        assert!(!ENVIRONMENT_VARIABLES.text_editor.is_empty());
    }
}
