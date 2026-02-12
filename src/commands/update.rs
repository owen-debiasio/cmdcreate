use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        sys::{
            ARCH, DistroBase, InstallMethod, VARS, arch_is_supported, args_forced, cpu_arch_check,
            get_distro_base, installation_method, run_shell_command,
        },
    },
    version::{VERSION, get_latest_commit, get_latest_release, is_development_version},
};

use std::{path::Path, process::exit};

pub fn get_install_path() -> &'static Path {
    ["/usr/bin/cmdcreate", "/usr/local/bin/cmdcreate"]
        .iter()
        .map(Path::new)
        .find(|p| p.exists())
        .unwrap()
}

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    let latest_release = &get_latest_release().unwrap_or_else(|| VERSION.to_owned());

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

    match installation_method(Option::from(get_install_path())) {
        InstallMethod::Other => interactive_upgrade(latest_release),

        InstallMethod::Aur => {
            if !args_forced()
                && input(&format!(
                "\n{blue}Arch Linux{reset}-based system detected. Would you like to install through the {blue}AUR{reset}?\n({green}Y{reset} or {red}N{reset})"
            ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                interactive_upgrade(latest_release);

                return;
            }

            upgrade_aur(!args_forced()
                && input(&format!(
                "\nWould you like to install the latest git {green}(commit: {}){reset}?\n({green}Y{reset} or {red}N{reset})", get_latest_commit("owen-debiasio", "cmdcreate", "main")))
                .trim()
                .eq_ignore_ascii_case("y"));
        }

        InstallMethod::Dpkg => {
            if arch_is_supported() {
                log("commands/update::update(): Debian/Ubuntu detected...", 0);

                if !args_forced()
                && input(&format!(
                "\n{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. Would you like to install via a {blue}.deb{reset} file?\n({green}Y{reset} or {red}N{reset})"
            ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                interactive_upgrade(latest_release);

                return;
            }

                upgrade_deb(latest_release);
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
                .eq_ignore_ascii_case("n")
            {
                interactive_upgrade(latest_release);

                return;
            }

                upgrade_rpm(latest_release);
            }
        }
    }

    exit(0)
}

fn upgrade_aur(git: bool) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let pkg = if git { "cmdcreate-git" } else { "cmdcreate" };

    delete_folder(&format!("{}/{pkg}", VARS.home));

    run_shell_command(&format!(
        "git clone https://aur.archlinux.org/{pkg}.git /tmp/{pkg} && \
         cd /tmp/{pkg} && \
         makepkg -si --noconfirm"
    ));

    delete_folder(&format!("{}/tmp/{pkg}", VARS.home));

    println!("{green}Update complete!{reset}");
}

fn upgrade_deb(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.deb");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo dpkg -i /tmp/{pkg}"
    ));

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_rpm(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.rpm");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo rpm -Uvh /tmp/{pkg}"
    ));

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_binary(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}-bin");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo install -Dm755 /tmp/{pkg} /usr/local/bin/cmdcreate"
    ));

    println!("\n{green}Update complete!{reset}");
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

fn interactive_upgrade(latest_release: &str) {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    let latest_commit: &str = &get_latest_commit("owen-debiasio", "cmdcreate", "main");

    println!("\nSelect an upgrade method:\n");

    for (mut i, option) in [
        &format!("Upgrade through AUR {blue}(universal device compatibility){reset}"),
        &format!("Upgrade through AUR {blue}(latest git {green}(commit: {latest_commit}){blue}, \
         universal device compatibility){reset}"),
        "Install via .deb file",
        "Install via .rpm file",
        "Manually install binary",
        &format!("Build from source {blue}(latest git {green}(commit: {latest_commit}){blue}, \
          universal device compatibility, {red}DEBIAN/UBUNTU MAY INVOLVE MANUAL INTERVENTION{blue}){reset}"),
        "Exit",
    ]
        .iter()
        .enumerate()
    {
        i += 1;
        println!("{blue}{i}]{reset} {option}");
    }

    match input("").trim() {
        "1" => upgrade_aur(false),
        "2" => upgrade_aur(true),
        "3" => upgrade_deb(latest_release),
        "4" => upgrade_rpm(latest_release),
        "5" => upgrade_binary(latest_release),
        "6" => build_from_source(),
        "7" => error("Aborted.", ""),

        _ => error("Invalid selection.", ""),
    }

    exit(0)
}

pub fn check() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log(
        "commands/update::check_for_updates(): Beginning update check...",
        0,
    );

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
