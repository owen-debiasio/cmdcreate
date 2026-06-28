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
    core::{
        configs::{init::init_configs, load::load_configuration},
        logger::{consts::Severity, main::log},
        meta::{
            project::author_information::AUTHOR,
            version::{build::get_build_status, consts::CURRENT_PROJECT_VERSION},
        },
    },
    utils::{
        fs::{init::init_filesystem, paths::CMDCREATE_BINARY_PATH},
        io::error,
        net::{internet_is_forced_disabled, not_connected_to_internet},
        sys::{
            cpu::ARCH,
            distro::{get_distro_base, installation_method, is_immutable_distro},
            env::{ENVIRONMENT_VARIABLES, running_as_root},
        },
    },
};

#[must_use]
pub fn debug_intro() -> String {
    let author_name = AUTHOR.name;
    let author_email = AUTHOR.email;

    let binary_path = CMDCREATE_BINARY_PATH.to_string();

    let build_status = get_build_status();

    let distro_base = get_distro_base();
    let immutable_distro_status = is_immutable_distro();
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
Binary path: {binary_path}
CPU architecture: {ARCH}
Distro base: {distro_base:?}
Distro is immutable: {immutable_distro_status}
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
    if is_immutable_distro() && running_as_root() {
        error(
            "Because you are using an immutable distro, you are unable to run cmdcreate as root.",
            None,
        )
    }

    init_filesystem();
    init_configs();

    if load_configuration("self", "disable_root_usage", "false") == "true" && running_as_root() {
        error(
            "Root usage is disabled on your machine!",
            Some("The setting 'disable_root_usage' is enabled in: '/etc/cmdcreate.toml'."),
        )
    }

    log(
        &format!(
            "core::init::init(): Starting cmdcreate...\n         {}",
            debug_intro()
        ),
        Severity::Normal,
    );
}
