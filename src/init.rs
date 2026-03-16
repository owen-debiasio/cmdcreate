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
    CURRENT_PROJECT_VERSION,
    configs::init_configs,
    logger::log,
    meta::{AUTHOR, AUTHOR_EMAIL},
    utils::{
        fs::init_fs_layout,
        io::error,
        net::is_offline,
        sys::{ARCH, VARS, get_distro_base, installation_method, is_root},
    },
    version::version_is_development_build,
};

pub fn debug_intro() -> String {
    format!(
        "
----------------
Welcome to cmdcreate!                   
Created by: {AUTHOR} <{AUTHOR_EMAIL}>
----------------
Have an issue? Copy the text below           
and open an issue                       
----------------
Version: {CURRENT_PROJECT_VERSION} {}
CPU architecture: {ARCH}
Distro base: {:?}
Installation Method: {:?}
Internet status: {}
Preferred text editor: {}
Shell in use: {}
----------------
",
        if version_is_development_build() {
            "(devel)"
        } else {
            "(stable)"
        },
        get_distro_base(),
        installation_method(),
        if is_offline() { "offline" } else { "connected" },
        VARS.editor,
        VARS.shell,
    )
}

pub fn init() {
    if !is_root() {
        error("Please run cmdcreate as root.", "")
    }

    init_fs_layout().expect("Failed to initialize filesystem");
    init_configs();

    log(
        &format!(
            "init::init(): Starting cmdcreate...\n         {}",
            debug_intro()
        ),
        0,
    );
}
