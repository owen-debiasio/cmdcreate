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
    input,
    logger::{Severity, log},
    meta::{author_information::AUTHOR, project_information::PROJECT},
    output, run_shell_command,
    utils::{
        colors::COLORS,
        fs::{
            core::{delete_file, delete_folder, path_exists},
            misc::{clone_repository, download_file_to_location_via_curl, install_binary},
        },
        io::{ask_for_confirmation, error, output_is_silent},
        net::not_connected_to_internet,
        sys::{
            cpu::{ARCH, arch_is_supported, cpu_arch_check},
            distro::{DistroBase, InstallMethod, get_distro_base, installation_method},
            env::{root_check, running_as_root},
        },
    },
    version::{get_latest_commit_from_repo, get_latest_tag_from_repo},
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

    if output_is_silent() {
        ask_for_confirmation(
            "I seriously don't recommend running the update \
             command silently, do you want to continue?",
            true,
        );
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
                Do you want to update via the AUR?"
            );

            if ask_for_confirmation(aur_install_confirmation, false) {
                update_via_aur();
            }
        }

        InstallMethod::Dpkg => {
            let deb_install_confirmation = &format!(
                "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. \
                Would you like to install via a {blue}.deb{reset} file?"
            );

            if ask_for_confirmation(deb_install_confirmation, false) {
                update_via_package(".deb");
            }
        }

        InstallMethod::Rpm => {
            let rpm_install_confirmation = &format!(
                "\n{blue}Fedora{reset}-based system detected. \
                Would you like to install via a {blue}.rpm{reset} file?"
            );

            if ask_for_confirmation(rpm_install_confirmation, false) {
                update_via_package(".rpm");
            }
        }

        InstallMethod::Other => (),
    }

    interactive_upgrade();
}

fn update_via_aur() {
    let (blue, magenta, green) = (COLORS.blue, COLORS.magenta, COLORS.green);

    if running_as_root() {
        error(
            "Please de-escalate from root to update using this method!",
            "You can't use the AUR when running from root.",
        )
    }

    let package_name: &str =
        if ask_for_confirmation("Do you want to install the latest git?", false) {
            "cmdcreate-git"
        } else {
            "cmdcreate"
        };

    output!(
        "{blue}Updating... {magenta}Please wait as this will take a while",
        true
    );

    let command_to_install = format!(
        "set -e

        sudo rm -rf /usr/bin/cmdcreate /tmp/cmdcreate_aur_tmp

        git clone https://aur.archlinux.org/{package_name}.git /tmp/cmdcreate_aur_tmp

        cd /tmp/cmdcreate_aur_tmp

        makepkg -si --noconfirm -- --overwrite /usr/bin/cmdcreate
        ",
    );

    run_shell_command!("{command_to_install}");

    delete_folder("/tmp/cmdcreate_aur_tmp");

    if !path_exists("/usr/bin/cmdcreate") {
        error("Update failed:", "Binary not found.")
    }

    output!("\n{green}Update complete!");

    exit(0)
}

/// Only handles `.deb`, `.rpm`, and `binary` files. Updating via the `AUR` is handled separately.
/// This is due to installing via the `AUR` requiring to not be run as root.
fn update_via_package(package_type: &str) {
    let green = COLORS.green;

    root_check();

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_stable_release = get_latest_tag_from_repo(author_username, project_name);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using \
         CPU Architectures other than \"x86_64\"!",
    );

    let package_file_name =
        &format!("cmdcreate-{latest_stable_release}-linux-{ARCH}{package_type}");
    let temp_package_file_path = &format!("/tmp/{package_file_name}");

    let project_repo = PROJECT.repository;
    let package_file_download_path =
        &format!("{project_repo}/releases/latest/download/{package_file_name}");

    download_file_to_location_via_curl(temp_package_file_path, package_file_download_path);

    match package_type {
        ".deb" => run_shell_command!("dpkg -i {temp_package_file_path}"),
        ".rpm" => run_shell_command!("rpm -Uvh {temp_package_file_path}"),
        "-bin" => install_binary("-Dm755", temp_package_file_path, "/usr/bin/cmdcreate"),

        _ => error(
            "Developer error: INVALID METHOD: (YOU SHOULDN'T BE ABLE TO SEE THIS)",
            package_type,
        ),
    }

    delete_file(temp_package_file_path);

    println!("\n{green}Update complete!");

    exit(0)
}

fn build_from_source() {
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
            cargo git less openssl curl base-devel"
        }
        DistroBase::Debian => {
            "apt update && \
            apt install -y \
            git less build-essential pkg-config libssl-dev curl"
        }
        DistroBase::Fedora => {
            "dnf update && \
            dnf install -y \
            git-core less openssl-devel pkgconf-pkg-config"
        }
        DistroBase::Unknown => error("Your distro is unsupported!", "Unable to proceed."),
    };

    output!(
        "\nUpdating cmdcreate {magenta}(please wait as this might take a while){blue}...",
        true
    );

    clone_repository(cloned_repository_destination);

    let script_to_build_cmdcreate = format!(
        "set -e
        {dependency_install_command}
        {rustup_install_command}
        
        # Source from HOME to be more resilient than hardcoded /root/
        [ -f \"$HOME/.cargo/env\" ] && . \"$HOME/.cargo/env\"
        
        cd \"{cloned_repository_destination}\"
        
        echo -e \"{blue}> Building... please wait...{reset}\"
        
        # --locked ensures the build uses your exact dependency versions
        cargo build --release --locked",
    );

    run_shell_command!("{script_to_build_cmdcreate}");

    output!("\nInstalling...", true);

    install_binary(
        "-Dm755",
        &format!("{cloned_repository_destination}/target/release/cmdcreate"),
        "/usr/bin/cmdcreate",
    );

    if !path_exists("/usr/bin/cmdcreate") {
        error("Failed to update!", "Updated binary not found!")
    }

    output!("\n{green}Update complete!{reset}");

    exit(0)
}

fn interactive_upgrade() {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    let author_username = AUTHOR.username;
    let project_name = PROJECT.name;

    let latest_commit = get_latest_commit_from_repo(author_username, project_name, "main");

    let installed_distro = get_distro_base();
    let cpu_arch_is_supported = arch_is_supported();

    let debian_build_warning = if installed_distro == DistroBase::Debian {
        format!(", {red}MAY INVOLVE MANUAL INTERVENTION{blue}")
    } else {
        String::new()
    };

    let compatibility_notice = if cpu_arch_is_supported {
        ""
    } else {
        ", universal compatibility"
    };

    output!("\nSelect an available upgrade method:\n", true);

    let mut chosen_update_method = Vec::new();

    if installed_distro == DistroBase::Arch {
        chosen_update_method.push((
            "aur",
            format!(
                "\
            Update via AUR{blue} \
            {compatibility_notice}",
            ),
        ));
    }

    if installed_distro == DistroBase::Debian && cpu_arch_is_supported {
        chosen_update_method.push(("deb", "Install via .deb file".to_string()));
    }
    if installed_distro == DistroBase::Fedora && cpu_arch_is_supported {
        chosen_update_method.push(("rpm", "Install via .rpm file".to_string()));
    }
    if cpu_arch_is_supported {
        chosen_update_method.push(("bin", "Install via raw binary".to_string()));
    }

    chosen_update_method.push((
        "src",
        format!(
            "\
            Build from source{blue} \
            (latest git {green}(commit: {latest_commit}){blue}\
            {compatibility_notice}\
            {debian_build_warning}){reset}",
        ),
    ));

    chosen_update_method.push(("exit", "Exit".to_string()));

    for (update_option_index, (_, update_option)) in chosen_update_method.iter().enumerate() {
        println!("{blue}{}]{reset} {update_option}", update_option_index + 1);
    }

    let entered_update_method = input!("").trim().parse::<usize>().unwrap_or(0);

    if chosen_update_method.is_empty() || entered_update_method == 0 {
        error("Please pick an option.", "")
    }

    if entered_update_method > chosen_update_method.len() {
        error("Invalid selection.", "");
    }

    match chosen_update_method[entered_update_method - 1].0 {
        "deb" => update_via_package(".deb"),
        "rpm" => update_via_package(".rpm"),
        "bin" => update_via_package("-bin"),
        "aur" => update_via_aur(),
        "src" => build_from_source(),
        "exit" => println!("Aborted."),
        _ => error("Unexpected error. Please try again.", ""),
    }
}
