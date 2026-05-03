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

use crate::utils::sys::env::running_as_root;
use crate::{
    CURRENT_PROJECT_VERSION,
    configs::init_configs,
    logger::{Severity, log},
    meta::author_information::AUTHOR,
    utils::{
        fs::init_filesystem,
        net::{internet_is_forced_disabled, not_connected_to_internet},
        sys::{
            cpu::ARCH,
            distro::{get_distro_base, installation_method},
            env::ENVIRONMENT_VARIABLES,
        },
    },
    version::get_build_status,
};

pub fn debug_intro() -> String {
    let author_name = AUTHOR.name;
    let author_email = AUTHOR.email;

    let build_status = get_build_status();

    let distro_base = get_distro_base();
    let installation_method = installation_method();

    let internet_status = if not_connected_to_internet() {
        if internet_is_forced_disabled() {
            "offline (forced)"
        } else {
            "offline"
        }
    } else {
        "connected"
    };

    let root_status = running_as_root();

    let chosen_text_editor = &ENVIRONMENT_VARIABLES.text_editor;
    let shell_in_use = &ENVIRONMENT_VARIABLES.shell;

    format!(
        "
----------------
Welcome to cmdcreate!
Created by: {author_name} <{author_email}>
----------------
Have an issue? Copy the text below
and open an issue
----------------
Version: {CURRENT_PROJECT_VERSION} {build_status}
CPU architecture: {ARCH}
Distro base: {distro_base:?}
Installation Method: {installation_method:?}
Internet status: {internet_status}
Preferred text editor: {chosen_text_editor}
Shell in use: {shell_in_use}
Root status: {root_status}
----------------
"
    )
}

pub fn init() {
    init_filesystem();
    init_configs();

    log(
        &format!(
            "init::init(): Starting cmdcreate...\n         {}",
            debug_intro()
        ),
        Severity::Normal,
    );
}
