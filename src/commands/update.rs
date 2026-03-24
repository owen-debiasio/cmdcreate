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

use crate::{
    logger::log,
    meta::{AUTHOR_USERNAME, PROJECT_NAME},
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

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?", true);

    if !arch_is_supported() {
        log(
            "commands/update::update(): ARM/Unsupported arch detected, switching to interactive...",
            0,
        );

        interactive_upgrade();
    }

    match installation_method() {
        InstallMethod::Aur => {
            let aur_install_confirmation = &format!(
                "\n{blue}Arch Linux{reset}-based system detected. Updating via AUR is not directly supported here. \
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

fn update_using_method(method: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let latest_stable_release = get_latest_tag_from_repo(AUTHOR_USERNAME, PROJECT_NAME);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    let package_file_name = &format!("cmdcreate-{latest_stable_release}-linux-{ARCH}{method}");
    let temp_package_file_path = &format!("/tmp/{package_file_name}");
    let package_file_download_path = &format!(
        "https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{package_file_name}"
    );

    download_file_to_location_via_curl(temp_package_file_path, package_file_download_path);

    match method {
        ".deb" => run_shell_command(&format!("dpkg -i {temp_package_file_path}")),
        ".rpm" => run_shell_command(&format!("rpm -Uvh {temp_package_file_path}")),
        "-bin" => run_shell_command(&format!(
            "install -Dm755 {temp_package_file_path} /usr/bin/cmdcreate"
        )),

        _ => error(
            "Developer error: INVALID METHOD: (YOU SHOULDN'T BE ABLE TO SEE THIS)",
            method,
        ),
    }

    delete_file(temp_package_file_path).expect("Failed to delete temp package file");

    println!("\n{green}Update complete!{reset}");

    exit(0)
}

/// This function is so messy and shitty so bear with me
/// TODO: Refactor this pile of garbage
fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let cache_dir: &str = "/root/.cache/cmdcreate";
    delete_folder(cache_dir).expect("Failed to delete folder");

    // The return values for DistroBase::Debian and DistroBase::Fedora are like the same fucking thing

    let install_cmd = match get_distro_base() {
        DistroBase::Arch => "sudo pacman -S --needed --noconfirm cargo git",
        DistroBase::Debian => {
            "sudo apt update && sudo apt install -y git build-essential pkg-config libssl-dev curl && \
            if ! command -v cargo >/dev/null; then curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; fi"
        }
        DistroBase::Fedora => {
            "sudo dnf install -y git-core openssl-devel pkgconf-pkg-config && \
            if ! command -v cargo >/dev/null; then curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; fi"
        }
        DistroBase::Unknown => {
            error(
                "Your system currently isn't supported for building from source.",
                "",
            );
        }
    };

    run_shell_command(&format!(
        "{install_cmd}
        set -e
        [ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"
        rm -rf {cache_dir}
        git clone https://github.com/owen-debiasio/cmdcreate.git \"{cache_dir}\"
        cd \"{cache_dir}\"
        sudo rustup default stable
        cargo build --release
        sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate"
    ));

    println!("\n{green}Update complete!{reset}");
}

fn interactive_upgrade() {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    println!("\nSelect an available upgrade method:\n");

    let mut available_update_methods = Vec::new();
    let distro = get_distro_base();

    if distro == DistroBase::Debian && arch_is_supported() {
        available_update_methods.push(("deb", "Install via .deb file".to_string()));
    }
    if distro == DistroBase::Fedora && arch_is_supported() {
        available_update_methods.push(("rpm", "Install via .rpm file".to_string()));
    }

    if arch_is_supported() {
        available_update_methods.push(("bin", "Manually install binary".to_string()));
    }

    let latest_git_repo_commit = get_latest_commit_from_repo(AUTHOR_USERNAME, PROJECT_NAME, "main");
    available_update_methods.push((
        "src",
        format!(
            "Build from source {blue}(latest git {green}(commit: {latest_git_repo_commit}){blue}, \
            universal device compatibility{}){reset}",
            if get_distro_base() == DistroBase::Debian {
                format!(", {red}MAY INVOLVE MANUAL INTERVENTION{blue}")
            } else {
                String::new()
            },
        ),
    ));

    available_update_methods.push(("exit", "Exit".to_string()));

    for (available_option_index, (_, text_of_option_that_gets_printed)) in
        available_update_methods.iter().enumerate()
    {
        println!(
            "{blue}{index_of_available_option}]{reset} {text_of_option_that_gets_printed}",
            index_of_available_option = available_option_index + 1
        );
    }

    let selection = input("").trim().parse::<usize>().unwrap_or(0);

    if selection == 0 || selection > available_update_methods.capacity() {
        error("Invalid selection: ", &selection.to_string());
    }

    match available_update_methods[selection - 1].0 {
        "deb" => update_using_method(".deb"),
        "rpm" => update_using_method(".rpm"),
        "bin" => update_using_method("-bin"),
        "src" => build_from_source(),
        "exit" => error("Aborted.", ""),
        _ => error("Invalid selection.", ""),
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

    let latest_stable_version = get_latest_tag_from_repo(AUTHOR_USERNAME, PROJECT_NAME);
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
