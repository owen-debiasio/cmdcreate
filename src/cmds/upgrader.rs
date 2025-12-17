use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::{
    error::Error,
    fs::{self, File},
    path::Path,
    process::Command,
};

use crate::{
    VERSION,
    utils::{colors::*, fs::*, msgs::*, sys::*},
};

/// Represents the installation method of cmdcreate
#[derive(Debug)]
pub enum InstallMethod {
    Aur,              // Installed via AUR (git clone + makepkg)
    Dpkg,             // Installed via .deb package
    Rpm,              // Installed via .rpm package
    StandaloneBinary, // Manual binary installation
}

/// Detects how cmdcreate was installed based on the system and package databases
pub fn installation_method(path: &Path) -> InstallMethod {
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return InstallMethod::StandaloneBinary,
    };
    let path_str = path.to_str().unwrap_or_default();

    // Determine base distro
    let distro_base = fs::read_to_string("/etc/os-release")
        .ok()
        .map(|s| {
            let s = s.to_lowercase();
            if s.contains("arch") || s.contains("manjaro") {
                "arch"
            } else if s.contains("fedora") || s.contains("rhel") || s.contains("centos") {
                "fedora"
            } else if s.contains("debian") || s.contains("ubuntu") || s.contains("linuxmint") {
                "debian"
            } else {
                "unknown"
            }
        })
        .unwrap_or("unknown");

    match distro_base {
        "arch" => {
            if Command::new("pacman")
                .args(["-Qo", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return InstallMethod::Aur;
            }
        }
        "fedora" => {
            if Command::new("rpm")
                .args(["-qf", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return InstallMethod::Rpm;
            }
        }
        "debian" => {
            if Command::new("dpkg-query")
                .args(["-S", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return InstallMethod::Dpkg;
            }
        }
        _ => {}
    }

    InstallMethod::StandaloneBinary
}

/// GitHub release structure
#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

/// GitHub release asset structure
#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

/// Performs the upgrade of cmdcreate
pub fn upgrade() {
    let latest_release = get_latest_release().unwrap_or_else(|| VERSION.to_string());
    let method = installation_method(Path::new("/usr/bin/cmdcreate"));

    match method {
        InstallMethod::Aur => upgrade_aur(),
        InstallMethod::Dpkg => upgrade_deb(&latest_release),
        InstallMethod::Rpm => upgrade_rpm(&latest_release),
        InstallMethod::StandaloneBinary => println!("Manual binary installation will be used."),
    }

    interactive_upgrade(latest_release);
}

/// Upgrade via AUR (git clone + makepkg)
fn upgrade_aur() {
    run_shell_command(
        "
        sudo rm /usr/bin/cmdcreate; \
        git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/cmdcreate; \
        cd ~/cmdcreate; \
        makepkg -si",
    );
    delete_folder(&format!("{}/cmdcreate", VARS.home));
}

/// Upgrade via .deb package
fn upgrade_deb(latest_release: &str) {
    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.deb \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.deb; \
         sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.deb"
    ));
}

/// Upgrade via .rpm package
fn upgrade_rpm(latest_release: &str) {
    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.rpm; \
         sudo rpm -Uvh /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm"
    ));
}

/// Fallback interactive upgrade for manual choice
fn interactive_upgrade(latest_release: String) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    ask_for_confirmation("Do you want to upgrade cmdcreate?");
    println!(
        "\nSelect an upgrade method:\n\n{blue}1]{reset} Upgrade through AUR\n{blue}2]{reset} Install via .deb file\n{blue}3]{reset} Install via .rpm file\n{blue}4]{reset} Manually install binary"
    );

    let mut method_input = String::new();
    std::io::stdin().read_line(&mut method_input).unwrap();

    match method_input.trim() {
        "1" => upgrade_aur(),
        "2" => upgrade_deb(&latest_release),
        "3" => upgrade_rpm(&latest_release),
        "4" => upgrade_binary(&latest_release),
        _ => error("Invalid selection.", ""),
    }
}

/// Upgrade by downloading the binary manually
fn upgrade_binary(latest_release: &str) {
    let (owner, repo) = ("owen-debiasio", "cmdcreate");
    let file_to_download = format!("cmdcreate-{latest_release}-linux-bin");
    let client = Client::new();

    let release: Release = client
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "reqwest")
        .send()
        .unwrap()
        .json()
        .unwrap();

    if let Some(asset) = release
        .assets
        .into_iter()
        .find(|a| a.name == file_to_download)
    {
        let tmp_path = format!("/tmp/{}", asset.name);
        let out_path = format!("/usr/bin/{}", asset.name);

        std::io::copy(
            &mut client.get(&asset.browser_download_url).send().unwrap(),
            &mut File::create(&tmp_path).unwrap(),
        )
        .unwrap();

        run_shell_command(&format!(
            "
            sudo mv +x {tmp_path} {out_path}; \
            sudo chmod +x {out_path}; \
            sudo rm /usr/bin/cmdcreate; \
            sudo mv /usr/bin/{file_to_download} /usr/bin/cmdcreate"
        ));

        println!(
            "Downloaded {} from release {}",
            asset.name, release.tag_name
        );
    } else {
        error("Binary not found in latest release.", "");
    }
}

/// Get the latest release tag from GitHub
pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let res = Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "rust-client")
        .send()?;

    let json: Value = res.json()?;
    Ok(json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_string())
}

/// Get the latest release version of cmdcreate
pub fn get_latest_release() -> Option<String> {
    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

/// Checks for updates and triggers the upgrade
pub fn check_for_updates() {
    println!("\nChecking for updates...");
    match get_latest_release() {
        Some(latest) if latest != VERSION => {
            println!(
                "{}Update available: {VERSION} -> {latest}{}",
                COLORS.green, COLORS.reset
            );
            upgrade();
        }
        Some(_) => println!("Already up to date."),
        None => error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        ),
    }
}
