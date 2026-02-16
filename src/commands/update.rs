use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        net::connected_to_internet,
        sys::{
            ARCH, DistroBase, InstallMethod, VARS, arch_is_supported, args_forced, cpu_arch_check,
            get_distro_base, installation_method, run_shell_command,
        },
    },
    version::{VERSION, get_latest_commit, get_latest_release, is_development_version},
};

use std::process::exit;

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    if !connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

    match installation_method() {
        InstallMethod::Aur => {
            if !args_forced()
                && input(&format!(
                "\n{blue}Arch Linux{reset}-based system detected. Updating on Arch-based distros using this method is not supported. \
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
                log("commands/update::update(): Debian/Ubuntu detected...", 0);

                if !args_forced()
                && input(&format!(
                "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. Would you like to install via a {blue}.deb{reset} file?\n({green}Y{reset} or {red}N{reset})"
            ))
                .trim()
                .eq_ignore_ascii_case("y")
            {
                interactive_upgrade();

                return;
            }

                upgrade_via("deb");
            }
        }

        InstallMethod::Rpm => {
            if arch_is_supported() {
                if !args_forced()
                && input(&format!(
                "\n{blue}Fedora{reset}-based system detected. Would you like to install via a {blue}.rpm{reset} file?\
                 \n({green}Y{reset} or {red}N{reset})"
            ))
                .trim()
                .eq_ignore_ascii_case("y")
            {
                interactive_upgrade();

                return;
            }

                upgrade_via("rpm");
            }
        }

        InstallMethod::Other => interactive_upgrade(),
    }
}

fn upgrade_via(method: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let latest_release = get_latest_release().unwrap_or_else(|| VERSION.to_owned());

    match method {
        "deb" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.deb");

            run_shell_command(&format!(
                "curl -Lf -o /tmp/{pkg} \
                 https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
                 sudo dpkg -i /tmp/{pkg} && \
                 rm /tmp/{pkg}"
            ));

            println!("\n{green}Update complete!{reset}");
        }
        "rpm" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.rpm");

            run_shell_command(&format!(
                "curl -Lf -o /tmp/{pkg} \
                 https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
                 sudo rpm -Uvh /tmp/{pkg} \
                 rm /tmp/{pkg}"
            ));

            println!("\n{green}Update complete!{reset}");
        }
        "bin" => {
            cpu_arch_check(
                "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
            );

            let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}-bin");

            run_shell_command(&format!(
                "curl -Lf -o /tmp/{pkg} \
                 https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
                 sudo install -Dm755 /tmp/{pkg} /usr/bin/cmdcreate \
                 rm /tmp/{pkg}"
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
    delete_folder(&cache_dir);

    run_shell_command(match get_distro_base() {
        DistroBase::Arch => "sudo pacman -S --needed --noconfirm cargo git",
        DistroBase::Debian => "sudo apt install -y cargo git build-essential pkg-config libssl-dev",
        DistroBase::Fedora => "sudo dnf install -y cargo git-core openssl-devel openssl-libs",
        DistroBase::Unknown => {
            error(
                "Your system currently isn't supported for building from source.",
                "",
            );
        }
    });

    run_shell_command(&format!(
        "git clone https://github.com/owen-debiasio/cmdcreate {cache_dir}",
    ));

    run_shell_command(&format!(
        "set -e && \
         cd \"{cache_dir}\" && \
         cargo build --release && \
         sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate",
    ));

    println!("\n{green}Update complete!{reset}");
}
fn interactive_upgrade() {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    println!("\nSelect an available upgrade method:\n");

    let mut options = Vec::new();
    let distro = get_distro_base();

    if distro == DistroBase::Debian {
        options.push(("deb", "Install via .deb file".to_string()));
    }
    if distro == DistroBase::Fedora {
        options.push(("rpm", "Install via .rpm file".to_string()));
    }

    options.push(("bin", "Manually install binary".to_string()));
    options.push(("src", format!(
        "Build from source {blue}(latest git {green}(commit: {}){blue}, \
        universal device compatibility, {red}DEBIAN/UBUNTU MAY INVOLVE MANUAL INTERVENTION{blue}){reset}"
    , get_latest_commit("owen-debiasio", "cmdcreate", "main"))));
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

    if !connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            "",
        )
    }

    println!("\nChecking for updates...");

    let Some(latest) = get_latest_release() else {
        error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        );
    };

    if is_development_version() {
        println!(
            "\nYou are running a newer version {}({VERSION}){reset} than the latest release {green}({latest}){reset}.\
                \nAssuming it's a development build.",
            COLORS.blue,
        );

        exit(0)
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
