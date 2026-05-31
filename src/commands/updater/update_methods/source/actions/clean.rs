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
    commands::updater::update_methods::source::dependencies::DEPENDENCIES_TO_INSTALL,
    output, run_shell_command,
    utils::{
        fs::core::delete_folder,
        io::{ask_for_confirmation, error},
        sys::distro::{DistroBase, get_distro_base},
    },
};

pub fn cleanup() {
    output!("Cleaning up...", true);

    ask_for_confirmation("Do you want to remove unneeded dependencies?", false);

    let dependencies = DEPENDENCIES_TO_INSTALL.to_string();

    let dependency_removal_command = match get_distro_base() {
        DistroBase::Arch => format!("pacman -Rns --noconfirm {dependencies}"),
        DistroBase::Debian => format!(
            "apt remove -y {}",
            dependencies.replace("zig", "").replace("rustup", "")
        ),
        DistroBase::Fedora => format!("dnf remove -y {}", dependencies.replace("rustup", "")),
        DistroBase::Unknown => error("Your distro is unsupported! Unable to proceed.", None),
    };

    if !dependencies.is_empty() {
        output!("Removing dependencies...", true);
        run_shell_command!("{dependency_removal_command}");
    }

    if dependencies.contains("rustup") && get_distro_base() != DistroBase::Arch {
        output!("Removing rustup...", true);

        output!("Removing \"/root/.cargo\"...", false);
        delete_folder("/root/.cargo");

        output!("Removing \"/root/.rustup\"...", false);
        delete_folder("/root/.rustup");
    }

    if dependencies.contains("zig") && get_distro_base() == DistroBase::Debian
        || get_distro_base() == DistroBase::Unknown
    {
        output!("Removing zig...", true);
        delete_folder("/tmp/cmdcreate-zig-tmp");
    }
}
