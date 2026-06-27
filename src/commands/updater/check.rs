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
    commands::updater::main::update,
    core::meta::version::{build::is_development_version, consts::CURRENT_PROJECT_VERSION},
    output,
    utils::{
        colors::COLORS, git::get_latest_tag, io::error, net::not_connected_to_internet,
        sys::distro::is_immutable_distro,
    },
};

pub fn check() {
    let (blue, green, magenta) = (COLORS.blue, COLORS.green, COLORS.magenta);

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            None,
        )
    }

    output!("\nChecking for updates...", true);

    let latest_stable_version = get_latest_tag();

    if is_development_version() {
        output!(
            "\nYou are running a newer version {magenta}({CURRENT_PROJECT_VERSION}){blue} \
            than the latest release {green}({latest_stable_version}).",
            true
        );

        return;
    }

    if CURRENT_PROJECT_VERSION != latest_stable_version {
        output!(
            "{green}\nUpdate available: {CURRENT_PROJECT_VERSION} -> {latest_stable_version}\n",
            true
        );

        if !is_immutable_distro() {
            update();
        }

        return;
    }

    output!("Already up to date.", true);
}
