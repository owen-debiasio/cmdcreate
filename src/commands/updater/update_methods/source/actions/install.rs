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
    commands::updater::update_methods::source::{
        consts::CLONED_REPOSITORY_DESTINATION, dependencies::Rustup,
    },
    output,
    utils::{
        fs::{misc::install_binary, paths::path_exists},
        io::error,
    },
};

pub fn install() {
    output!("\nInstalling...", true);

    install_binary(
        "-Dm755",
        &format!(
            "{CLONED_REPOSITORY_DESTINATION}/target/{}/release/cmdcreate",
            Rustup::target()
        ),
        "/usr/bin/cmdcreate",
    );

    if !path_exists("/usr/bin/cmdcreate") {
        error("Failed to update!", Some("Updated binary not found!"))
    }
}
