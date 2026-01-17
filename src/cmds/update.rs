use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, fs::File, io::copy, path::Path, process::exit};

use crate::{
    VERSION,
    logger::log,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        sys::{
            ARCH, DistroBase, InstallMethod, VARS, args_forced, get_distro_base,
            installation_method, run_shell_command,
        },
    },
};

#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn http_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("cmdcreate-upgrader")
        .build()
        .expect("Failed to build HTTP client")
}

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    log("cmds/update::update(): Initializing upgrade process...", 0);

    let latest_release = &get_latest_release().unwrap_or_else(|| VERSION.to_string());

    log(
        "cmds/update::update(): Determining installation method...",
        0,
    );

    log("cmds/update::update(): Getting installation paths...", 0);

    let install_path = ["/usr/bin/cmdcreate", "/usr/local/bin/cmdcreate"]
        .iter()
        .map(Path::new)
        .find(|p| p.exists());

    let Some(install_path) = install_path else {
        log(
            "cmds/update::update(): cmdcreate not found, falling back to interactive upgrade",
            1,
        );
        interactive_upgrade(latest_release);
        return;
    };

    match installation_method(install_path) {
        InstallMethod::Aur => {
            log("cmds/update::update(): Arch Linux detected...", 0);

            if !args_forced()
                && input(&format!(
                    "{blue}Arch Linux{reset}-based system detected. Would you like to install through the {blue}AUR{reset}?\n({green}Y{reset} or {red}N{reset})"
                ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                println!();
                interactive_upgrade(latest_release);
                return;
            }

            let use_git = !args_forced()
                && input(&format!(
                    "\nWould you like to install the latest git?\n({green}Y{reset} or {red}N{reset})"
                ))
                .trim()
                .eq_ignore_ascii_case("y");

            upgrade_aur(use_git);
        }

        InstallMethod::Dpkg => {
            log("cmds/update::update(): Debian/Ubuntu detected...", 0);

            if !args_forced()
                && input(&format!(
                    "{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. Would you like to install via a {blue}.deb{reset} file?\n({green}Y{reset} or {red}N{reset})"
                ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                interactive_upgrade(latest_release);
                return;
            }

            upgrade_deb(latest_release);
        }

        InstallMethod::Rpm => {
            log("cmds/update::update(): Fedora detected...", 0);

            if !args_forced()
                && input(&format!(
                    "{blue}Fedora{reset}-based system detected. Would you like to install via a {blue}.rpm{reset} file?\n({green}Y{reset} or {red}N{reset})"
                ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                interactive_upgrade(latest_release);
                return;
            }

            upgrade_rpm(latest_release);
        }

        InstallMethod::Other => {
            log(
                "cmds/update::update(): No distro detected... Moving on to interactive upgrade...",
                1,
            );
            interactive_upgrade(latest_release);
        }
    }
}

fn upgrade_aur(git: bool) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    let pkg = if git { "cmdcreate-git" } else { "cmdcreate" };

    log("cmds/update::update_aur(): Installing from AUR...", 0);

    delete_folder(&format!("{}/{pkg}", VARS.home));

    run_shell_command(&format!(
        "git clone https://aur.archlinux.org/{pkg}.git ~/{pkg} && \
         cd ~/{pkg} && \
         makepkg -si --noconfirm"
    ));

    delete_folder(&format!("{}/{pkg}", VARS.home));

    log("cmds/update::update_aur(): Update completed.", 0);

    println!("{green}Update complete!{reset}");
}

fn upgrade_deb(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log("cmds/update::update_deb(): Installing .deb...", 0);

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.deb");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo dpkg -i /tmp/{pkg}"
    ));

    log("cmds/update::update_deb(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_rpm(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log("cmds/update::update_rpm(): Installing .rpm...", 0);

    let pkg = format!("cmdcreate-{latest_release}-linux-{ARCH}.rpm");

    run_shell_command(&format!(
        "curl -Lf -o /tmp/{pkg} \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/{pkg} && \
         sudo rpm -Uvh /tmp/{pkg}"
    ));

    log("cmds/update::update_rpm(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_binary(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log("cmds/update::update_binary(): Fetching release info...", 0);

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
            exit(1);
        });

    let tmp = format!("/tmp/{}", asset.name);

    log("cmds/update::update_binary(): Downloading binary...", 0);

    copy(
        &mut http_client()
            .get(&asset.browser_download_url)
            .send()
            .expect("Download failed"),
        &mut File::create(&tmp).expect("Temp file failed"),
    )
    .expect("Write failed");

    log("cmds/update::update_binary(): Installing binary...", 0);

    run_shell_command(&format!("sudo install -Dm755 {tmp} /usr/bin/cmdcreate"));

    log("cmds/update::update_binary(): Update completed.", 0);

    println!("\n{green}Update complete!{reset}");
}

fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log(
        "cmds/update::build_from_source(): Deleting temp folder (if exists)...",
        0,
    );

    delete_folder(&format!("{}/.cache/cmdcreate", VARS.home));

    log(
        "cmds/update::build_from_source(): Retrieving distro base...",
        0,
    );

    let pm: &str = match get_distro_base() {
        DistroBase::Arch => "sudo pacman -S --noconfirm",
        DistroBase::Debian => "sudo apt install -y",
        DistroBase::Fedora => "sudo dnf install -y",
        DistroBase::Unknown => {
            error(
                "Your system currently isn't supported for building from source.",
                "",
            );
            exit(1);
        }
    };

    log(
        &format!("cmds/update::build_from_source(): Detected package manager as \"{pm}\"..."),
        0,
    );

    log(
        "cmds/update::build_from_source(): Installing dependencies...",
        0,
    );

    run_shell_command(&format!("{pm} cargo git"));

    log("cmds/update::build_from_source(): Cloning repository...", 0);

    run_shell_command("git clone https://github.com/owen-debiasio/cmdcreate ~/.cache/cmdcreate");

    log(
        "cmds/update::build_from_source(): Building and installing package...",
        0,
    );

    run_shell_command(
        "set -e && \
         cd \"$HOME/.cache/cmdcreate\" && \
         cargo update -p cmdcreate --precise \"$VERSION\" && \
         cargo build --release && \
         sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate",
    );

    log("cmds/update::build_from_source(): Update complete...", 0);

    println!("\n{green}Update complete!{reset}");
}

fn interactive_upgrade(latest_release: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log(
        "cmds/update::interactive_upgrade(): Initializing interactive upgrade...",
        0,
    );

    log(
        "cmds/update::interactive_upgrade(): Requesting permission to upgrade...",
        0,
    );

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    log(
        "cmds/update::interactive_upgrade(): Continuing with upgrade...",
        0,
    );

    log(
        "cmds/update::interactive_upgrade(): Providing upgrade options...",
        0,
    );

    println!("\nSelect an upgrade method:\n");

    for (i, option) in [
        &format!("Upgrade through AUR {blue}(universal device compatibility){reset}"),
        &format!("Upgrade through AUR {blue}(latest git, universal device compatibility){reset}"),
        "Install via .deb file",
        "Install via .rpm file",
        "Manually install binary",
        &format!("Build from source {blue}(latest git, universal device compatibility){reset}"),
        "Exit",
    ]
    .iter()
    .enumerate()
    {
        println!("{blue}{}]{reset} {option}", i + 1);
    }

    log(
        "cmds/update::interactive_upgrade(): Allowing user to select upgrade method...",
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
        "cmds/update::interactive_upgrade(): Interactive update completed...",
        0,
    );

    exit(0)
}

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    log("cmds/update::get_latest_tag(): Retrieving latest tag...", 0);

    let json: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .send()?
        .json()?;

    let tag = json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_string();

    log(
        &format!("cmds/update::get_latest_tag(): Latest tag: {tag}..."),
        0,
    );

    Ok(tag)
}

pub fn get_latest_release() -> Option<String> {
    log(
        "cmds/update::get_latest_release(): Retrieving latest release...",
        0,
    );

    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

pub fn check() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log(
        "cmds/update::check_for_updates(): Beginning update check...",
        0,
    );

    println!("\nChecking for updates...");

    let (current, latest) = (VERSION, get_latest_release().unwrap_or_default());

    log(
        &format!(
            "cmds/update::check_for_updates(): Comparing versions \"{current} (Current)\" to \"{latest}\" (Latest)..."
        ),
        0,
    );

    match get_latest_release() {
        Some(latest) if latest != VERSION => {
            log(
                &format!(
                    "cmds/update::check_for_updates(): Found available update from \"{current}\" to \"{latest}\"..."
                ),
                0,
            );

            println!("{green}\nUpdate available: {VERSION} -> {latest}{reset}");

            log(
                "cmds/update::check_for_updates(): Asking user for confirmation...",
                0,
            );

            ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

            log(
                "cmds/update::check_for_updates(): Launching upgrade process...",
                0,
            );

            update();
        }

        Some(_) => {
            log("cmds/update::check_for_updates(): No updates available.", 1);

            println!("Already up to date.");
        }

        None => error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        ),
    }
}

pub fn _get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    let res: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits/{branch}"
        ))
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    res["sha"].as_str().expect("missing sha")[..7].to_string()
}
