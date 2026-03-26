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

use std::process::exit;

use crate::logger::Severity;
use crate::{
    logger::log,
    meta::{author_information::AUTHOR, project_information::PROJECT},
    utils::{
        colors::COLORS,
        fs::{delete_file, delete_folder, download_file_to_location_via_curl},
        io::{ask_for_confirmation, error, input},
        net::not_connected_to_internet,
        sys::{
            ARCH, DistroBase, InstallMethod, arch_is_supported, cpu_arch_check, get_distro_base,
            installation_method, run_shell_command,
        },
    },
    version::{
        CURRENT_PROJECT_VERSION, get_latest_commit_from_repo, get_latest_tag_from_repo,
        version_is_development_build,
    },
};

pub fn update() {
    let (blue, red, reset) = (COLORS.blue, COLORS.red, COLORS.reset);

    let project_name = PROJECT.name;

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    let question_to_ask = &format!("\nDo you want to upgrade {project_name}?");
    ask_for_confirmation(question_to_ask, true);

    if !arch_is_supported() {
        log(
            "commands/update::update(): ARM/Unsupported arch detected, switching to interactive...",
            Severity::Normal,
        );

        interactive_upgrade();
    }

    match installation_method() {
        InstallMethod::Aur => {
            let aur_install_confirmation = &format!(
                "\n{blue}Arch Linux{reset}-based system detected. \
                Updating via AUR is not directly supported here. \
                Do you want to use the interactive update instead?"
            );

            if ask_for_confirmation(aur_install_confirmation, false) {
                interactive_upgrade();
            }

            error("Aborted.", "")
        }

        InstallMethod::Dpkg => {
            let deb_install_confirmation = &format!(
                "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. \
                Would you like to install via a {blue}.deb{reset} file?"
            );

            if ask_for_confirmation(deb_install_confirmation, false) {
                update_using_method(".deb");
            }

            interactive_upgrade();
        }

        InstallMethod::Rpm => {
            let rpm_install_confirmation = &format!(
                "\n{blue}Fedora{reset}-based system detected. \
                Would you like to install via a {blue}.rpm{reset} file?"
            );

            if ask_for_confirmation(rpm_install_confirmation, false) {
                update_using_method(".rpm");
            }

            interactive_upgrade();
        }

        InstallMethod::Other => interactive_upgrade(),
    }
}

fn update_using_method(method_for_installation: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_stable_release = get_latest_tag_from_repo(author_username, project_name);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    let package_file_name =
        &format!("cmdcreate-{latest_stable_release}-linux-{ARCH}{method_for_installation}");
    let temp_package_file_path = &format!("/tmp/{package_file_name}");
    let package_file_download_path = &format!(
        "https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{package_file_name}"
    );

    download_file_to_location_via_curl(temp_package_file_path, package_file_download_path);

    match method_for_installation {
        ".deb" => run_shell_command(&format!("dpkg -i {temp_package_file_path}")),
        ".rpm" => run_shell_command(&format!("rpm -Uvh {temp_package_file_path}")),
        "-bin" => run_shell_command(&format!(
            "install -Dm755 {temp_package_file_path} /usr/bin/cmdcreate"
        )),

        _ => error(
            "Developer error: INVALID METHOD: (YOU SHOULDN'T BE ABLE TO SEE THIS)",
            method_for_installation,
        ),
    }

    delete_file(temp_package_file_path).expect("Failed to delete temp package file");

    println!("\n{green}Update complete!{reset}");

    exit(0)
}

fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let cloned_repository_destination = "/root/.cache/cmdcreate";

    delete_folder(cloned_repository_destination).expect("Failed to clear cache");

    let rustup_install_command = "
        if ! command -v cargo >/dev/null; then \
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; \
        fi";

    let dependency_install_command = match get_distro_base() {
        DistroBase::Arch => {
            "pacman -Sy && pacman -S --needed --noconfirm \
            cargo git openssl curl base-devel"
        }
        DistroBase::Debian => {
            "apt update && apt install -y \
            git build-essential pkg-config libssl-dev curl"
        }
        DistroBase::Fedora => {
            "dnf update && dnf install -y \
            git-core openssl-devel pkgconf-pkg-config"
        }
        DistroBase::Unknown => error("Your distro is unsupported! Unable to proceed.", ""),
    };

    let project_repo = PROJECT.repository;

    let script_to_build_cmdcreate = format!(
        "{dependency_install_command} && {rustup_install_command}
        set -e
        [ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"
        git clone {project_repo}.git \"{cloned_repository_destination}\"
        cd \"{cloned_repository_destination}\"
        rustup default stable
        cargo build --release
        install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate",
    );

    run_shell_command(&script_to_build_cmdcreate);

    println!("\n{green}Update complete!{reset}");

    exit(0)
}

fn interactive_upgrade() {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_commit = get_latest_commit_from_repo(author_username, project_name, "main");

    let installed_distro = get_distro_base();
    let cpu_arch_is_supported = arch_is_supported();

    println!("\nSelect an available upgrade method:\n");

    let mut chosen_update_method = Vec::new();

    if installed_distro == DistroBase::Debian && cpu_arch_is_supported {
        chosen_update_method.push(("deb", "Install via .deb file".to_string()));
    }
    if installed_distro == DistroBase::Fedora && cpu_arch_is_supported {
        chosen_update_method.push(("rpm", "Install via .rpm file".to_string()));
    }
    if cpu_arch_is_supported {
        chosen_update_method.push(("bin", "Manually install binary".to_string()));
    }

    let debian_build_warning = if installed_distro == DistroBase::Debian {
        format!(", {red}MAY INVOLVE MANUAL INTERVENTION{blue}")
    } else {
        String::new()
    };

    chosen_update_method.push((
        "src",
        format!(
            "Build from source {blue}(latest git {green}(commit: {latest_commit}){blue}, \
        universal compatibility{debian_build_warning}){reset}"
        ),
    ));

    chosen_update_method.push(("exit", "Exit".to_string()));

    for (update_option_index, (_, update_option)) in chosen_update_method.iter().enumerate() {
        println!("{blue}{}]{reset} {update_option}", update_option_index + 1);
    }

    let entered_update_method = input("").trim().parse::<usize>().unwrap_or(0);

    if entered_update_method == 0 || entered_update_method > chosen_update_method.len() {
        error("Invalid selection: ", &entered_update_method.to_string());
    }

    match chosen_update_method[entered_update_method - 1].0 {
        "deb" => update_using_method(".deb"),
        "rpm" => update_using_method(".rpm"),
        "bin" => update_using_method("-bin"),
        "src" => build_from_source(),
        "exit" => println!("Aborted."),
        _ => error("Unexpected error. Please try again.", ""),
    }
}

pub fn check() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    println!("\nChecking for updates...");

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_stable_version = get_latest_tag_from_repo(author_username, project_name);
    let current_version = CURRENT_PROJECT_VERSION;

    if version_is_development_build() {
        println!(
            "\nYou are running a newer version {}({current_version}){reset} \
            than the latest release {green}({latest_stable_version}){reset}.",
            COLORS.blue
        );

        return;
    }

    if current_version != latest_stable_version {
        println!(
            "{green}\nUpdate available: {current_version} -> {latest_stable_version}{reset}\n"
        );

        update();

        return;
    }

    println!("Already up to date.");
}
