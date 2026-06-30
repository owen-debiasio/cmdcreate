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
    commands::updater::{
        interactive::interactive_upgrade,
        update_methods::{aur::update_via_aur, package::update_via_package},
    },
    core::{
        logger::{consts::Severity, main::log},
        meta::project::project_information::PROJECT,
    },
    utils::{
        colors::COLORS,
        io::{ask_for_confirmation, error, output_is_silent},
        net::not_connected_to_internet,
        sys::{
            cpu::arch_is_supported,
            distro::{get_distro_base, is_immutable_distro},
        },
    },
};

fn init() {
    if is_immutable_distro() {
        error(
            "Because you are using an immutable distro, you are unable to update cmdcreate using the built-in command.",
            None,
        )
    }

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            None,
        )
    }

    if output_is_silent() {
        ask_for_confirmation(
            "I seriously don't recommend running the update \
             command silently, do you want to continue?",
            true,
        );
    }
}

pub fn update() {
    let (blue, red, reset) = (COLORS.blue, COLORS.red, COLORS.reset);

    init();

    let project_name = PROJECT.name;
    let question_to_ask = &format!("\nDo you want to upgrade {project_name}?");
    ask_for_confirmation(question_to_ask, true);

    if !arch_is_supported() {
        log(
            "commands/update::update(): ARM/Unsupported arch detected, switching to interactive...",
            Severity::Normal,
        );

        interactive_upgrade();
    }

    match get_distro_base() {
        "Arch" => {
            let aur_install_confirmation = &format!(
                "\n{blue}Arch Linux{reset}-based system detected. \
                Do you want to update via the AUR?"
            );

            if ask_for_confirmation(aur_install_confirmation, false) {
                update_via_aur();
            }
        }

        "Debian" => {
            let deb_install_confirmation = &format!(
                "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. \
                Would you like to install via a {blue}.deb{reset} file?"
            );

            if ask_for_confirmation(deb_install_confirmation, false) {
                update_via_package(".deb");
            }
        }

        "Fedora" => {
            let rpm_install_confirmation = &format!(
                "\n{blue}Fedora{reset}-based system detected. \
                Would you like to install via a {blue}.rpm{reset} file?"
            );

            if ask_for_confirmation(rpm_install_confirmation, false) {
                update_via_package(".rpm");
            }
        }

        _ => (),
    }

    interactive_upgrade();
}
