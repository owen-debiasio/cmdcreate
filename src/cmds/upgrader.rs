/// Module for handling system updates and version management in cmdcreate
///
/// This module provides functionality for checking, downloading, and installing
/// updates to the cmdcreate system. It supports multiple installation methods
/// including AUR, .deb packages, and direct binary installation.
///
/// The module interfaces with GitHub's API to check for new releases and
/// handles the download and installation of updates through various package
/// managers and installation methods.
use std::{error::Error, fs::File};

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    utils::{
        colors::*, // Terminal color formatting
        fs::*,     // File system operations
        msgs::*,   // User messaging
        sys::*,    // System operations
    },
    PROJ_VER, // Current project version
};

/// Represents a GitHub release with its tag and assets
#[derive(Deserialize)]
struct Release {
    tag_name: String,   // Version tag of the release
    assets: Vec<Asset>, // List of downloadable assets in the release
}

/// Represents a downloadable asset in a GitHub release
#[derive(Deserialize)]
struct Asset {
    name: String,                 // Name of the asset file
    browser_download_url: String, // Direct download URL for the asset
}

/// Initiates the upgrade process for cmdcreate
///
/// This function:
/// 1. Gets the latest release version
/// 2. Prompts for upgrade confirmation
/// 3. Offers multiple installation methods:
///    - AUR (yay or paru)
///    - Debian package (.deb)
///    - RPM package (.rpm)
///    - Direct binary installation
///
/// # Installation Methods
/// - AUR: Both package manager and manual clone options
/// - Debian: Downloads and installs .deb package
/// - RPM: Downloads and installs .rpm package
/// - Binary: Direct download and installation
pub fn upgrade() {
    let (blue, red, yellow, reset) = (COLORS.blue, COLORS.red, COLORS.yellow, COLORS.reset);

    let latest_release = get_latest_release().unwrap().to_string();

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    println!(
        "\nSelect an upgrade method:\n\n{blue}1]{reset} Upgrade through AUR\n{blue}2]{reset} Install via .deb file\n{blue}3]{reset} Install via .rpm file\n{blue}4]{reset} Manually install binary"
    );

    let mut method = String::new();
    std::io::stdin().read_line(&mut method).unwrap();

    match method.trim() {
        "1" => {
            println!(
                "\nSelect a download method:\n\n{blue}1]{reset} AUR Manager {yellow}(yay {red}and {yellow}paru {red}ONLY)\n{blue}2]{reset} Clone repository {blue}(Recommended for when the AUR is down){reset}"
            );

            let mut method = String::new();
            std::io::stdin().read_line(&mut method).unwrap();

            match method.trim() {
                "1" => {
                    println!(
                        "\nSelect an AUR Manager:\n\n{blue}1]{reset} yay\n{blue}2]{reset} paru\n{blue}"
                    );

                    let mut method = String::new();
                    std::io::stdin().read_line(&mut method).unwrap();

                    match method.trim() {
                        "1" | "2" => run_shell_command(&format!("{method} -Syyu")),
                        _ => error("Invalid selection.", ""),
                    }
                }
                "2" => {
                    run_shell_command(
                            "
                            sudo rm /usr/bin/cmdcreate; \
                            git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git cmdcreate; \
                            cd ~/cmdcreate; \
                            makepkg -si");
                    delete_folder(&format!("{}/cmdcreate", VARS.home));
                }

                _ => error("Invalid selection.", ""),
            }
        }
        "2" => {
            println!("Downloading latest Debian package...");

            run_shell_command(
                &format!(
                    "
                    curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.deb https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.deb; \
                    sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.deb;
                    "
                ));
        }
        "3" => {
            println!("Downloading latest RPM package...");

            run_shell_command(
                &format!(
                    "
                    curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.rpm; \
                    sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm;
                    "
                ));
        }
        "4" => {
            println!("Downloading latest binary...");

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
                let (tmp_path, out_path) = (
                    &format!("/tmp/{}", asset.name),
                    &format!("/usr/bin/{}", asset.name),
                );

                std::io::copy(
                    &mut client.get(&asset.browser_download_url).send().unwrap(),
                    &mut File::create(tmp_path).unwrap(),
                )
                .unwrap();

                run_shell_command(&format!(
                    "
                        sudo mv +x {tmp_path} {out_path}; \
                        sudo chmod +x {out_path}; \
                        sudo rm /usr/bin/cmdcreate; \
                        sudo mv /usr/bin/{file_to_download} /usr/bin/cmdcreate
                        ",
                ));

                println!(
                    "Downloaded {} from release {}",
                    asset.name, release.tag_name
                )
            } else {
                error("Binary not found in latest release.", "");
            }
        }
        _ => error("Invalid selection.", ""),
    }
}

/// Retrieves the latest release tag from a GitHub repository
///
/// # Arguments
/// * `owner` - The GitHub repository owner
/// * `repo` - The repository name
///
/// # Returns
/// * `Result<String, Box<dyn Error>>` - The latest release tag or an error
///
/// # Errors
/// Returns an error if:
/// - API request fails
/// - Response parsing fails
/// - Tag name is missing from response
pub fn get_latest_tag(owner: &str, repo: &str) -> Result<String, Box<dyn Error>> {
    let res = reqwest::blocking::Client::new()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest"
        ))
        .header("User-Agent", "rust-client")
        .send()?;

    let json: Value = res.json()?;
    let tag = json["tag_name"]
        .as_str()
        .ok_or("Missing tag_name in response")?
        .to_string();

    Ok(tag)
}

/// Gets the latest release version of cmdcreate
///
/// # Returns
/// * `Option<String>` - The latest release version or None if fetch fails
pub fn get_latest_release() -> Option<String> {
    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

/// Checks for available updates and initiates upgrade if needed
///
/// This function:
/// 1. Fetches the latest release version
/// 2. Compares with current version
/// 3. Initiates upgrade process if newer version exists
pub fn check_for_updates() {
    println!("\nChecking for updates...");
    match get_latest_release() {
        Some(latest) if latest != PROJ_VER => {
            println!(
                "{}Update available: {PROJ_VER} -> {latest}{}",
                COLORS.green, COLORS.reset
            );
            upgrade();
        }
        Some(_) => println!("Already up to date."),
        None => error(
            "Failed to check for updates.\nMake sure you are connected to the internet.\n",
            "",
        ),
    }
}
