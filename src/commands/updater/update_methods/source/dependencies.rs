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
    core::{
        configs::load::load_configuration,
        logger::{consts::Severity, main::log},
    },
    output, run_shell_command,
    utils::{
        fs::core::creation::{delete_file, delete_folder},
        io::error,
        sys::{
            command::system_command_is_installed, cpu::ARCH, distro::get_distro_base,
            env::ENVIRONMENT_VARIABLES,
        },
    },
};

pub static DEPENDENCIES_TO_INSTALL: LazyLock<String> = LazyLock::new(|| {
    // Rustup and zig are needed, but they are installed manually (without a package manager)
    let needed_dependencies = vec!["git", "curl", "less", "wget"];
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

    output!("Downloading and installing rustup...", true);
    Rustup::install();

    output!("Downloading and installing zig...", true);
    install_zig();

    if dependencies.is_empty() {
        return;
    }

    log(
        &format!(
            "commands::updater::update_methods::source::dependencies::install_dependencies(): Dependencies to install: {dependencies}"
        ),
        Severity::Normal,
    );

    let dependency_install_command = match get_distro_base() {
        "Arch" => format!(
            "pacman -Sy && pacman -S --needed --noconfirm \
              {dependencies}",
        ),
        "Debian" => format!(
            "apt update && \
              apt install -y \
           {dependencies}",
        ),
        "Fedora" => format!(
            "dnf update && \
           dnf install -y \
              {dependencies}",
        ),
        _ => error("Your distro is unsupported! Unable to proceed.", None),
    };

    output!("Installing dependencies...", true);
    run_shell_command!("{dependency_install_command}");
}

pub fn get_cargo_env() -> &'static str {
    match ENVIRONMENT_VARIABLES.shell.as_str() {
        "fish" => "source \"$HOME/.cargo/env.fish\"",
        "nushell" => "source \"~/.cargo/env.nu\"",
        "tcsh" => "source \"$HOME/.cargo/env.tcsh\"",
        "xonsh" => "source \"$HOME/.cargo/env.xsh\"",

        // Bash is the default cause most distros use it
        _ => ". \"$HOME/.cargo/env\"",
    }
}

fn install_zig() {
    let zig_version = load_configuration("update", "zig_version", "0.16.0");

    log(
        &format!(
            "cmdcreate::commands::updater::update_methods::source::dependencies::install_zig(): Using zig version: {zig_version}"
        ),
        Severity::Normal,
    );

    // The version is hardcoded to download, I'll update it if needed
    let zig_download_page = &format!("https://ziglang.org/download/{zig_version}");

    log(
        &format!(
            "cmdcreate::commands::updater::update_methods::source::dependencies::install_zig(): Using zig download page: {zig_download_page}"
        ),
        Severity::Normal,
    );

    let zig_download_file = match ARCH {
        "x86_64" => &format!("{zig_download_page}/zig-x86_64-linux-{zig_version}.tar.xz"),
        "i686" | "i386" => &format!("{zig_download_page}/zig-x86-linux-{zig_version}.tar.xz"),
        "aarch64" | "arm64" => {
            &format!("{zig_download_page}/zig-aarch64-linux-{zig_version}.tar.xz")
        }
        "armv7" | "armv7l" => &format!("{zig_download_page}/zig-arm-linux-{zig_version}.tar.xz"),
        _ => error("Unsupported architecture:", Some(ARCH)),
    };

    log(
        &format!(
            "cmdcreate::commands::updater::update_methods::source::dependencies::install_zig(): Downloading zig from: {zig_download_file}"
        ),
        Severity::Normal,
    );

    if !system_command_is_installed("wget") {
        error(
            "Failed to download zig:",
            Some("Command \"wget\" is not installed."),
        );
    }
    if !system_command_is_installed("tar") {
        error(
            "Failed to extract zig:",
            Some("Command \"tar\" is not installed."),
        );
    }

    let zig_archive_name = zig_download_file
        .replace(&format!("{zig_download_page}/"), "")
        .trim()
        .to_string();

    let commands_to_install_zig = &format!(
        "wget -P /tmp/ {zig_download_file} && \
         mkdir -p /tmp/cmdcreate-zig-tmp && \
         tar -xf /tmp/{zig_archive_name} -C /tmp/cmdcreate-zig-tmp --strip-components=1"
    );

    run_shell_command!("{commands_to_install_zig}");

    delete_file(&format!("/tmp/{zig_archive_name}"));
}

pub struct Rustup;

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

    pub fn install() {
        run_shell_command!(
            "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable",
        );

        run_shell_command!("{}", get_cargo_env());
    }

    pub fn uninstall() {
        // The command "rustup" doesn't work here so I manually call for the respective directories to be removed.

        output!("Removing \"/root/.cargo\"...", false);
        delete_folder("/root/.cargo");

        output!("Removing \"/root/.rustup\"...", false);
        delete_folder("/root/.rustup");

        output!("Removing \"/root/.cache/cargo-zigbuild\"...", false);
        delete_folder("/root/.cache/cargo-zigbuild");
    }
}
