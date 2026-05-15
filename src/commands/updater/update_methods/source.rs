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
    output, run_shell_command,
    utils::{
        colors::COLORS,
        fs::{
            core::{delete_folder, path_exists},
            misc::{clone_repository, install_binary},
        },
        io::error,
        sys::{
            cpu::ARCH,
            distro::{DistroBase, get_distro_base},
            env::root_check,
        },
    },
};
use std::process::exit;

pub fn build() {
    let (blue, magenta, green, reset) = (COLORS.blue, COLORS.magenta, COLORS.green, COLORS.reset);

    root_check();

    let cloned_repository_destination = "/tmp/cmdcreate";

    delete_folder(cloned_repository_destination);

    let rustup_install_command = "
        if ! command -v cargo >/dev/null; then \
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable; \
        fi";

    let dependency_install_command = match get_distro_base() {
        DistroBase::Arch => {
            "pacman -Sy && pacman -S --needed --noconfirm \
            cargo git less openssl curl base-devel zig"
        }
        DistroBase::Debian => {
            "apt update && \
            apt install -y \
            git less build-essential pkg-config libssl-dev curl zig"
        }
        DistroBase::Fedora => {
            "dnf update && \
            dnf install -y \
            git-core less openssl-devel pkgconf-pkg-config zig"
        }
        DistroBase::Unknown => error("Your distro is unsupported! Unable to proceed.", None),
    };

    output!(
        "\nUpdating cmdcreate {magenta}(please wait as this might take a while){blue}...",
        true
    );

    clone_repository(cloned_repository_destination);

    let (rust_target, zig_target, cc_var) = match ARCH {
        "x86_64" => (
            "x86_64-unknown-linux-musl",
            "x86_64-linux-musl",
            "CC_x86_64_unknown_linux_musl",
        ),
        "i686" | "i386" => (
            "i686-unknown-linux-musl",
            "x86-linux-musl",
            "CC_i686_unknown_linux_musl",
        ),
        "aarch64" | "arm64" => (
            "aarch64-unknown-linux-musl",
            "aarch64-linux-musl",
            "CC_aarch64_unknown_linux_musl",
        ),
        "armv7" | "armv7l" => (
            "armv7-unknown-linux-musleabihf",
            "arm-linux-musleabihf",
            "CC_armv7_unknown_linux_musleabihf",
        ),
        _ => error(&format!("Unsupported architecture: {ARCH}"), None),
    };

    let script_to_build_cmdcreate = format!(
        "set -e
        {dependency_install_command}
        {rustup_install_command}

        [ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"

        rustup target add {rust_target}
        if ! command -v cargo-zigbuild >/dev/null; then
            cargo install cargo-zigbuild
        fi

        cd \"{cloned_repository_destination}\"

        echo -e \"{blue}> Building ({ARCH})... please wait...{reset}\"

        export CRATE_CC_NO_DEFAULTS=true
        export {cc_var}=\"zig cc -target {zig_target} -fno-sanitize=all\"

        cargo zigbuild --release --locked --target {rust_target}",
    );

    run_shell_command!("{script_to_build_cmdcreate}");

    output!("\nInstalling...", true);

    install_binary(
        "-Dm755",
        &format!("{cloned_repository_destination}/target/{rust_target}/release/cmdcreate"),
        "/usr/bin/cmdcreate",
    );

    if !path_exists("/usr/bin/cmdcreate") {
        error("Failed to update!", Some("Updated binary not found!"))
    }

    output!("\n{green}Update complete!{reset}");

    exit(0)
}
