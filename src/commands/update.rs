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
    logger::log,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        net::is_offline,
        sys::{
            ARCH, DistroBase, InstallMethod, VARS, arch_is_supported, args_forced, cpu_arch_check,
            get_distro_base, installation_method, run_shell_command,
        },
    },
    version::{VERSION, get_latest_commit, get_latest_tag, is_development_version},
};

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    if is_offline() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

    match installation_method() {
        InstallMethod::Aur => {
            // These blocks are repeated. smh
            if !args_forced()
                && !input(&format!(
                    "\n{blue}Arch Linux{reset}-based system detected. Updating via AUR is not directly supported here. \
Do you want to use the interactive update instead?\n({green}Y{reset} or {red}N{reset})"
                ))
                .trim()
                .eq_ignore_ascii_case("y")
                {
                    interactive_upgrade();
                    return;
                }
            error("Aborted.", "")
        }

        InstallMethod::Dpkg => {
            if arch_is_supported() {
                if !args_forced() && !input(&format!(
                    "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. Would you like to install via a \
                    {blue}.deb{reset} file?\n({green}Y{reset} or {red}N{reset})"
                )).trim().eq_ignore_ascii_case("y") {
                    interactive_upgrade();
                    return;
                }
                upgrade_via("deb");
            } else {
                log(
                    "commands/update::update(): ARM/Unsupported arch detected, switching to interactive...",
                    0,
                );
                interactive_upgrade();
            }
        }

        InstallMethod::Rpm => {
            if arch_is_supported() {
                if !args_forced() && !input(&format!(
                    "\n{blue}Fedora{reset}-based system detected. Would you like to install via a \
                    {blue}.rpm{reset} file?\n({green}Y{reset} or {red}N{reset})"
                )).trim().eq_ignore_ascii_case("y") {
                    interactive_upgrade();
                    return;
                }
                upgrade_via("rpm");
            } else {
                interactive_upgrade();
            }
        }

        InstallMethod::Other => interactive_upgrade(),
    }
}

fn upgrade_via(method: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let latest_release = get_latest_tag("owen-debiasio", "cmdcreate");

    match method {
        /*
        let package_file = format!("cmdcreate-{latest_release}-linux-...");
        This is defined like 3 times, and I'm not 100% sure how to fix that
        */
        "deb" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let package_file = format!("cmdcreate-{latest_release}-linux-{ARCH}.deb");

            // These install "scripts" are also repeated 3 times.
            run_shell_command(&format!(
                "curl -Lf -o /tmp/{package_file} \
                https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{package_file} && \
                sudo dpkg -i /tmp/{package_file} && \
                rm /tmp/{package_file}"
            ));

            println!("\n{green}Update complete!{reset}");
        }
        "rpm" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let package_file = format!("cmdcreate-{latest_release}-linux-{ARCH}.rpm");

            run_shell_command(&format!(
                "curl -Lf -o /tmp/{package_file} \
                https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{package_file} && \
                sudo rpm -Uvh /tmp/{package_file} \
                rm /tmp/{package_file}"
            ));

            println!("\n{green}Update complete!{reset}");
        }
        "bin" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let package_file = format!("cmdcreate-{latest_release}-linux-{ARCH}-bin");

            run_shell_command(&format!(
                "curl -Lf -o /tmp/{package_file} \
                https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{package_file} && \
                sudo install -Dm755 /tmp/{package_file} /usr/bin/cmdcreate && \
                rm /tmp/{package_file}"
            ));

            println!("\n{green}Update complete!{reset}");
        }
        _ => error(
            "Developer error: INVALID METHOD: (YOU SHOULDN'T BE ABLE TO SEE THIS)",
            method,
        ),
    }
}

fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);
    let cache_dir = format!("{}/.cache/cmdcreate", VARS.home);

    delete_folder(&cache_dir).expect("Failed to delete folder");

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

    run_shell_command(install_cmd);

    run_shell_command(&format!(
        "set -e
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

    let mut options = Vec::new();
    let distro = get_distro_base();

    if distro == DistroBase::Debian && arch_is_supported() {
        options.push(("deb", "Install via .deb file".to_string()));
    }
    if distro == DistroBase::Fedora && arch_is_supported() {
        options.push(("rpm", "Install via .rpm file".to_string()));
    }

    if arch_is_supported() {
        options.push(("bin", "Manually install binary".to_string()));
    }

    options.push((
        "src",
        format!(
            "Build from source {blue}(latest git {green}(commit: {}){blue}, \
        universal device compatibility{}){reset}",
            get_latest_commit("owen-debiasio", "cmdcreate", "main"),
            if get_distro_base() == DistroBase::Debian {
                format!(", {red}MAY INVOLVE MANUAL INTERVENTION{blue}")
            } else {
                String::new()
            },
        ),
    ));

    options.push(("exit", "Exit".to_string()));

    for (i, (_, text)) in options.iter().enumerate() {
        println!("{blue}{idx}]{reset} {text}", idx = i + 1);
    }

    let selection = input("").trim().parse::<usize>().unwrap_or(0);

    if selection == 0 || selection > options.capacity() {
        error("Invalid selection: ", &selection.to_string());
    }

    match options[selection - 1].0 {
        "deb" => upgrade_via("deb"),
        "rpm" => upgrade_via("rpm"),
        "bin" => upgrade_via("bin"),
        "src" => build_from_source(),
        "exit" => error("Aborted.", ""),
        _ => error("Invalid selection.", ""),
    }
}

pub fn check() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    if is_offline() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    println!("\nChecking for updates...");

    let latest = get_latest_tag("owen-debiasio", "cmdcreate");

    if is_development_version() {
        println!(
            "\nYou are running a newer version {}({VERSION}){reset} \
            than the latest release {green}({latest}){reset}.",
            COLORS.blue
        );

        return;
    }

    if VERSION != latest {
        ask_for_confirmation(&format!(
            "{green}\nUpdate available: {VERSION} -> {latest}{reset}\nDo you want to upgrade cmdcreate?"
        ));

        update();

        return;
    }

    println!("Already up to date.");
}
