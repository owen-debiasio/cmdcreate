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
    commands::updater::update_methods::{
        aur::update_via_aur, package::update_via_package, source::main::source,
    },
    input, output,
    utils::{
        colors::COLORS,
        git::get_latest_commit,
        io::error,
        sys::{cpu::arch_is_supported, distro::get_distro_base},
    },
};

pub fn interactive_upgrade() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    output!("\nSelect an available upgrade method:\n", true);

    let mut chosen_update_method = Vec::new();

    let cpu_arch_is_supported = arch_is_supported();

    let installed_distro = get_distro_base();

    if installed_distro == "Arch" {
        chosen_update_method.push(("aur", "Update via AUR"));
    }

    if cpu_arch_is_supported {
        match installed_distro {
            "Debian" => chosen_update_method.push(("deb", "Update via .deb file")),
            "Fedora" => chosen_update_method.push(("rpm", "Update via .rpm file")),
            _ => (),
        }

        chosen_update_method.push(("bin", "Update via raw binary"));
    }

    let latest_commit = get_latest_commit();

    // I have to make this variable separate because Clippy yells at you otherwise
    let build_from_source_message = &format!("Build from source (commit: {latest_commit})");

    chosen_update_method.push(("src", build_from_source_message));

    chosen_update_method.push(("exit", "Exit"));

    for (update_option_index, (_, update_option)) in chosen_update_method.iter().enumerate() {
        println!("{blue}{}]{reset} {update_option}", update_option_index + 1);
    }

    let entered_update_method = input!("").trim().parse::<usize>().unwrap_or(0);

    if chosen_update_method.is_empty() || entered_update_method == 0 {
        error("Please pick an option.", None)
    }

    if entered_update_method > chosen_update_method.len() {
        error("Invalid selection.", None);
    }

    match chosen_update_method[entered_update_method - 1].0 {
        "deb" => update_via_package(".deb"),
        "rpm" => update_via_package(".rpm"),
        "bin" => update_via_package("-bin"),
        "aur" => update_via_aur(),
        "src" => source(),
        "exit" => error("Aborted.", None),
        _ => error("Unexpected error. Please try again.", None),
    }
}
