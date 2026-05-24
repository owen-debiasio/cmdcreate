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

use std::sync::LazyLock;

use crate::{
    output, run_shell_command,
    utils::{
        io::error,
        sys::{
            command::system_command_is_installed,
            cpu::ARCH,
            distro::{DistroBase, get_distro_base},
        },
    },
};

static DEPENDENCIES_TO_INSTALL: LazyLock<String> = LazyLock::new(|| {
    let needed_dependencies = vec!["git", "zig", "curl", "less"];
    let mut dependencies_to_install = Vec::new();

    for dep in needed_dependencies {
        if !system_command_is_installed(dep) {
            dependencies_to_install.push(dep);
        }
    }

    let mut list_of_dependencies = String::new();
    for dep in dependencies_to_install {
        list_of_dependencies = format!("{list_of_dependencies}{dep} ");
    }

    list_of_dependencies
});

pub fn install_dependencies() {
    let dependencies = DEPENDENCIES_TO_INSTALL.to_string();

    let dependency_install_command = match get_distro_base() {
        DistroBase::Arch => format!(
            "pacman -Sy && pacman -S --needed --noconfirm \
              {dependencies}"
        ),
        DistroBase::Debian => format!(
            "apt update && \
              apt install -y \
           {dependencies}"
        ),
        DistroBase::Fedora => format!(
            "dnf update && \
           dnf install -y \
              {dependencies}"
        ),
        DistroBase::Unknown => error("Your distro is unsupported! Unable to proceed.", None),
    };

    if !DEPENDENCIES_TO_INSTALL.is_empty() {
        output!("Installing dependencies...", true);
        run_shell_command!("{dependency_install_command}");
    }

    output!("Installing rustup...", true);
    run_shell_command!(
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        [ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"

        rustup target add {}
        cargo install cargo-zigbuild",
        Rustup::target()
    );
}

pub struct Rustup {}

impl Rustup {
    pub fn target() -> &'static str {
        match ARCH {
            "x86_64" => "x86_64-unknown-linux-musl",
            "i686" | "i386" => "i686-unknown-linux-musl",
            "aarch64" | "arm64" => "aarch64-unknown-linux-musl",
            "armv7" | "armv7l" => "armv7-unknown-linux-musleabihf",
            _ => error("Unsupported architecture:", Some(ARCH)),
        }
    }

    pub fn zig_target() -> &'static str {
        match ARCH {
            "x86_64" => "x86_64-linux-musl",
            "i686" | "i386" => "x86-linux-musl",
            "aarch64" | "arm64" => "aarch64-linux-musl",
            "armv7" | "armv7l" => "arm-linux-musleabihf",
            _ => error("Unsupported architecture:", Some(ARCH)),
        }
    }
    pub fn cc_linker() -> &'static str {
        match ARCH {
            "x86_64" => "CC_x86_64_unknown_linux_musl",
            "i686" | "i386" => "CC_i686_unknown_linux_musl",
            "aarch64" | "arm64" => "CC_aarch64_unknown_linux_musl",
            "armv7" | "armv7l" => "CC_armv7_unknown_linux_musleabihf",
            _ => error("Unsupported architecture:", Some(ARCH)),
        }
    }
}
