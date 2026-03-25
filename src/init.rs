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
    meta::author_information::AUTHOR,
    utils::{
        fs::init_fs_layout,
        io::{ask_for_confirmation, error},
        net::not_connected_to_internet,
        sys::{
            ARCH, ENVIRONMENT_VARIABLES, get_distro_base, installation_method,
            root_requirement_is_bypassed, running_as_root,
        },
    },
    version::version_is_development_build,
};

pub fn debug_intro() -> String {
    format!(
        "
----------------
Welcome to cmdcreate!                   
Created by: {} <{}>
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
        AUTHOR.name,
        AUTHOR.email,
        if version_is_development_build() {
            "(devel)"
        } else {
            "(stable)"
        },
        get_distro_base(),
        installation_method(),
        if not_connected_to_internet() {
            "offline"
        } else {
            "connected"
        },
        ENVIRONMENT_VARIABLES.text_editor,
        ENVIRONMENT_VARIABLES.shell,
    )
}

fn root_check() {
    let user_bypasses_root: bool = root_requirement_is_bypassed();
    let user_is_running_as_root: bool = running_as_root();

    if !user_is_running_as_root && !user_bypasses_root {
        error("Please run cmdcreate as root.", "")
    }

    if user_bypasses_root && !user_is_running_as_root {
        ask_for_confirmation(
            "Root requirement is bypassed, which means instability and incompataility will occur. Proceed?",
            true,
        );
    }
}

pub fn init() {
    root_check();

    // These will NOT work if user is not running as root.
    // Root access is required for the directories
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
