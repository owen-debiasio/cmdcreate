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
        consts::CLONED_REPOSITORY_DESTINATION,
        dependencies::{Rustup, get_cargo_env, install_dependencies},
    },
    output, run_shell_command,
    utils::{fs::core::delete_folder, git::clone_repository},
};

pub fn build() {
    delete_folder(CLONED_REPOSITORY_DESTINATION);
    clone_repository(CLONED_REPOSITORY_DESTINATION);

    install_dependencies();

    output!("Building...", true);

    let target = Rustup::target();

    let script_to_build_cmdcreate = format!(
        "set -e

        cd \"{CLONED_REPOSITORY_DESTINATION}\"
        
        {}

        rustup default stable
        rustup target add {target}
        cargo install cargo-zigbuild

        CRATE_CC_NO_DEFAULTS=true {}=\"zig cc -target {} -fno-sanitize=all\" \
        CARGO_ZIGBUILD_ZIG_PATH=\"/usr/bin/zig\" \
        cargo zigbuild --release --locked --target {target}
        ",
        get_cargo_env(),
        Rustup::cc_linker(),
        Rustup::zig_target(),
    );

    run_shell_command!("{script_to_build_cmdcreate}");
}
