use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        misc::http_client,
        sys::{
            ARCH, DistroBase, InstallMethod, VARS, arch_is_supported, args_forced, cpu_arch_check,
            get_distro_base, installation_method, run_shell_command,
        },
    },
    version::{VERSION, get_latest_commit, get_latest_release, is_development_version},
};

use serde::Deserialize;
use std::{fs::File, io::copy, path::Path, process::exit};

#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn get_install_path() -> &'static Path {
    ["/usr/bin/cmdcreate", "/usr/local/bin/cmdcreate"]
        .iter()
        .map(Path::new)
        .find(|p| p.exists())
        .unwrap()
}

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    log(
        "commands/update::update(): Initializing upgrade process...",
        0,
    );

    log("commands/update::update(): Retrieving latest release...", 0);

    let latest_release = &get_latest_release().unwrap_or_else(|| VERSION.to_owned());

    log(
        "commands/update::interactive_upgrade(): Requesting permission to upgrade...",
        0,
    );

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

    log(
        "commands/update::interactive_upgrade(): Continuing with upgrade...",
        0,
    );

    log(
        "commands/update::update(): Determining installation method...",
        0,
    );

    match installation_method(Option::from(get_install_path())) {
        InstallMethod::Aur => {
            log("commands/update::update(): Arch Linux detected...", 0);

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
                log("commands/update::update(): Fedora detected...", 0);

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

        InstallMethod::Other => {
            log(
                "commands/update::update(): No distro detected... Moving on to interactive upgrade...",
                1,
            );

            interactive_upgrade(latest_release);
        }
    }

    exit(0)
}

fn upgrade_aur(git: bool) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let pkg = if git { "cmdcreate-git" } else { "cmdcreate" };

    log("commands/update::update_aur(): Installing from AUR...", 0);

    delete_folder(&format!("{}/{pkg}", VARS.home));

    run_shell_command(&format!(
        "git clone https://aur.archlinux.org/{pkg}.git /tmp/{pkg} && \
         cd /tmp/{pkg} && \
         makepkg -si --noconfirm"
    ));

    delete_folder(&format!("{}/tmp/{pkg}", VARS.home));

    log("commands/update::update_aur(): Update completed.", 0);

    println!("{green}Update complete!{reset}");
}

fn upgrade_deb(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    log(
        "commands/update::update_deb(): Installing .deb package...",
        0,
    );

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.deb");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo dpkg -i /tmp/{pkg}"
    ));

    log("commands/update::update_deb(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_rpm(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    log(
        "commands/update::update_rpm(): Installing .rpm package...",
        0,
    );

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.rpm");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo rpm -Uvh /tmp/{pkg}"
    ));

    log("commands/update::update_rpm(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_binary(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    cpu_arch_check(
        "You cannot update cmdcreate via this method using CPU Architectures other than \"x86_64\"!",
    );

    log(
        "commands/update::update_binary(): Fetching release info...",
        0,
    );

    let release: Release = http_client()
        .get("https://api.github.com/repos/owen-debiasio/cmdcreate/releases/latest")
        .send()
        .expect("Failed to fetch release info")
        .json()
        .expect("Invalid release JSON");

    let asset = release
        .assets
        .into_iter()
        .find(|a| a.name == format!("cmdcreate-{latest_release}-linux-{ARCH}-bin"))
        .unwrap_or_else(|| {
            error("Binary not found in latest release.", "");
        });

    let tmp = format!("/tmp/{}", asset.name);

    log("commands/update::update_binary(): Downloading binary...", 0);

    copy(
        &mut http_client()
            .get(&asset.browser_download_url)
            .send()
            .expect("Download failed"),
        &mut File::create(&tmp).expect("Temp file failed"),
    )
    .expect("Write failed");

    log("commands/update::update_binary(): Installing binary...", 0);

    run_shell_command(&format!("sudo install -Dm755 {tmp} /usr/bin/cmdcreate"));

    log("commands/update::update_binary(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log(
        "commands/update::build_from_source(): Deleting temp folder (if exists)...",
        0,
    );

    let cache_dir = format!("{}/.cache/cmdcreate", VARS.home);
    delete_folder(&cache_dir);

    log(
        "commands/update::build_from_source(): Installing build dependencies...",
        0,
    );

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

    log(
        "commands/update::build_from_source(): Cloning repository...",
        0,
    );

    run_shell_command(&format!(
        "git clone https://github.com/owen-debiasio/cmdcreate {cache_dir}",
    ));

    log(
        "commands/update::build_from_source(): Building and installing package...",
        0,
    );

    run_shell_command(&format!(
        "set -e && \
         cd \"{cache_dir}\" && \
         cargo build --release && \
         sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate",
    ));

    log(
        "commands/update::build_from_source(): Update complete...",
        0,
    );

    println!("\n{green}Update complete!{reset}");
}

fn interactive_upgrade(latest_release: &str) {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    let latest_commit: &str = &get_latest_commit("owen-debiasio", "cmdcreate", "main");

    log(
        "commands/update::interactive_upgrade(): Initializing interactive upgrade...",
        0,
    );

    log(
        "commands/update::interactive_upgrade(): Providing upgrade options...",
        0,
    );

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

    log(
        "commands/update::interactive_upgrade(): Allowing user to select upgrade method...",
        0,
    );

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

    log(
        "commands/update::interactive_upgrade(): Interactive update completed...",
        0,
    );

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

    log(
        &format!(
            "commands/update::check_for_updates(): Comparing versions \"{VERSION} (Current)\" to \"{latest}\" (Latest)..."
        ),
        0,
    );

    if is_development_version() {
        log(
            "commands/update::check_for_updates(): Current version is newer than the latest release.",
            1,
        );

        println!(
            "\nYou are running a newer version {}({VERSION}){reset} than the latest release {green}({latest}){reset}.\
                \nAssuming it's a development build.",
            COLORS.blue,
        );

        exit(0)
    }

    if VERSION != latest {
        log(
            &format!(
                "commands/update::check_for_updates(): Found available update from \"{VERSION}\" to \"{latest}\"..."
            ),
            0,
        );

        println!("{green}\nUpdate available: {VERSION} -> {latest}{reset}");

        log(
            "commands/update::check_for_updates(): Asking user for confirmation...",
            0,
        );

        ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

        log(
            "commands/update::check_for_updates(): Launching upgrade process...",
            0,
        );

        update();

        return;
    }

    log(
        "commands/update::check_for_updates(): No updates available.",
        1,
    );

    println!("Already up to date.");
}
