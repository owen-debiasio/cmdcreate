use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, fs::File, io::copy, path::Path, process::exit};

use crate::{
    VERSION,
    utils::{
        colors::COLORS,
        fs::delete_folder,
        io::{ask_for_confirmation, error, input},
        sys::{
            InstallMethod, VARS, args_forced, get_distro_base, installation_method,
            run_shell_command,
        },
    },
};

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn upgrade() {
    let latest_release = &get_latest_release().unwrap_or_else(|| VERSION.to_string());
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    match installation_method(Path::new("/usr/bin/cmdcreate")) {
        InstallMethod::Aur => {
            if input(&format!(
                "{blue}Arch Linux{reset}-based system detected. Would you like to install through the {blue}AUR{blue}?\n({green}Y{reset} or {red}N{reset})"
            ))
            .trim()
            .to_lowercase()
                != "y" && !args_forced()
            {
                println!();

                interactive_upgrade(latest_release);

                return;
            }

            if input(&format!(
                "\nWould you like to install the latest git?\n({green}Y{reset} or {red}N{reset})"
            ))
            .trim()
            .to_lowercase()
                == "y"
                && !args_forced()
            {
                upgrade_aur(true);

                return;
            }

            upgrade_aur(false);
        }

        InstallMethod::Dpkg => {
            if input(&format!(
                "{red}Debian{reset}/{red}Ubuntu{reset}-based system detected. Would you like to install via a {blue}.deb{reset} file?\n({green}Y{reset} or {red}N{reset})"
            ))
            .trim()
            .to_lowercase()
                != "y" && !args_forced()
            {
                interactive_upgrade(latest_release);

                return;
            }

            upgrade_deb(latest_release);
        }

        InstallMethod::Rpm => {
            if input(&format!(
                "{blue}Fedora{reset}-based system detected. Would you like to install via a {blue}.rpm{reset} file?\n({green}Y{reset} or {red}N{reset})"
            ))
            .trim()
            .to_lowercase()
                != "y" && !args_forced()
            {
                interactive_upgrade(latest_release);

                return;
            }

            upgrade_rpm(latest_release);
        }

        InstallMethod::Other => interactive_upgrade(latest_release),
    }
}

fn upgrade_aur(git: bool) {
    let pkg = if git { "cmdcreate-git" } else { "cmdcreate" };

    delete_folder(&format!("{}/{pkg}", VARS.home));

    run_shell_command(&format!(
        "git clone --branch {pkg} --single-branch https://github.com/archlinux/aur.git ~/{pkg}; \
         cd ~/{pkg}; \
         makepkg -si",
    ));

    delete_folder(&format!("{}/{pkg}", VARS.home));
}

fn upgrade_deb(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.deb \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.deb; \
         sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.deb"
    ));

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_rpm(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.rpm; \
         sudo rpm -Uvh /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm"
    ));

    println!("\n{green}Update complete!{reset}");
}

fn upgrade_binary(latest_release: &str) {
    let client = Client::new();

    let release: Release = client
        .get("https://api.github.com/repos/owen-debiasio/cmdcreate/releases/latest")
        .header("User-Agent", "reqwest")
        .send()
        .expect("Failed to fetch release info")
        .json()
        .expect("Failed to parse release info");

    if let Some(asset) = release
        .assets
        .into_iter()
        .find(|a| a.name == format!("cmdcreate-{latest_release}-linux-bin"))
    {
        let tmp_path = format!("/tmp/{}", asset.name);

        copy(
            &mut client
                .get(&asset.browser_download_url)
                .send()
                .expect("Failed to download binary"),
            &mut File::create(&tmp_path).expect("Failed to create temp file"),
        )
        .expect("Failed to copy binary");

        run_shell_command(&format!(
            "sudo chmod +x {tmp_path}; \
             sudo mv {tmp_path} /usr/bin/cmdcreate"
        ));

        println!(
            "Downloaded {} from release {}",
            asset.name, release.tag_name
        );
    } else {
        error("Binary not found in latest release.", "");
    }
}

fn build_from_source() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    delete_folder(&format!("{}/.cache/cmdcreate", VARS.home));

    run_shell_command("git clone https://github.com/owen-debiasio/cmdcreate ~/.cache/cmdcreate");

    let pm: &str = match get_distro_base() {
        "arch" => "sudo pacman -S --noconfirm",
        "debian" => "sudo apt install -y",
        "fedora" => "sudo dnf install -y",

        _ => {
            error(
                "Your system currently isn't supported for building from source.",
                "",
            );

            exit(1);
        }
    };

    run_shell_command(&format!(
        "{pm} cargo && \
         cd ~/.cache/cmdcreate && \
         cargo build --release && \
         sudo cp target/release/cmdcreate /usr/bin/cmdcreate.new && \
         sudo chmod +x /usr/bin/cmdcreate.new && \
         sudo mv /usr/bin/cmdcreate.new /usr/bin/cmdcreate",
    ));

    println!("\n{green}Update complete!{reset}");
}

fn interactive_upgrade(latest_release: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

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

pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let json: Value = Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "rust-client")
        .send()?
        .json()?;

    Ok(json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_string())
}

pub fn get_latest_release() -> Option<String> {
    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

pub fn check_for_updates() {
    let (green, reset) = (COLORS.green, COLORS.reset);

    println!("\nChecking for updates...");

    match get_latest_release() {
        Some(latest) if latest != VERSION => {
            println!("{green}\nUpdate available: {VERSION} -> {latest}{reset}",);
            ask_for_confirmation("\nDo you want to upgrade cmdcreate?");
            upgrade();
        }

        Some(_) => println!("Already up to date."),

        None => error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        ),
    }
}

pub fn _get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    let res: Value = Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits/{branch}"
        ))
        .header("User-Agent", "rust-app")
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    res["sha"].as_str().expect("missing sha")[..7].to_string()
}
