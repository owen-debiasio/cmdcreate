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
    commands::updater::update_methods::source::dependencies::{DEPENDENCIES_TO_INSTALL, Rustup},
    output, run_shell_command,
    utils::{
        fs::core::creation::delete_folder,
        io::{ask_for_confirmation, error},
        sys::distro::{DistroBase, get_distro_base},
    },
};

pub fn cleanup() {
    output!("Cleaning up...", true);

    if !ask_for_confirmation("Do you want to remove unneeded dependencies?", false) {
        return;
    }

    let dependencies = DEPENDENCIES_TO_INSTALL.to_string();

    let dependency_removal_command = match get_distro_base() {
        DistroBase::Arch => format!("pacman -Rns --noconfirm {dependencies}"),
        DistroBase::Debian => format!("apt remove -y {dependencies}"),
        DistroBase::Fedora => format!("dnf remove -y {dependencies}"),
        DistroBase::Unknown => error("Your distro is unsupported! Unable to proceed.", None),
    };

    if !dependencies.is_empty() {
        output!("Removing dependencies...", true);
        run_shell_command!("{dependency_removal_command}");
    }

    output!("Removing rustup...", true);
    Rustup::uninstall();

    output!("Removing zig...", true);

    output!("Removing \"/tmp/cmdcreate-zig-tmp\"...", false);
    delete_folder("/tmp/cmdcreate-zig-tmp");

    output!("Removing \"/tmp/cmdcreate-zig-tmp\"...", false);
    delete_folder("/root/.cache/zig");
}
