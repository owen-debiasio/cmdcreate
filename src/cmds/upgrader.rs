use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::{
    error::Error,
    fs::File,
    io::{copy, stdin},
    path::Path,
    process::{Command, exit},
};

use crate::{
    VERSION,
    utils::{
        colors::COLORS,
        fs::{delete_folder, read_file_to_string},
        msgs::{ask_for_confirmation, error},
        sys::{VARS, run_shell_command},
    },
};

#[derive(Debug)]
pub enum InstallMethod {
    Aur,
    Dpkg,
    Rpm,
    Other,
}

pub fn installation_method(path: &Path) -> InstallMethod {
    let Ok(path) = path.canonicalize() else {
        return InstallMethod::Other;
    };

    let Some(path_str) = path.to_str() else {
        return InstallMethod::Other;
    };

    match get_distro_base() {
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

    InstallMethod::Other
}

fn get_distro_base() -> &'static str {
    let content = read_file_to_string("/etc/os-release").to_lowercase();

    let mut id: &str = "";
    let mut id_like: &str = "";

    for line in content.lines() {
        if let Some(v) = line.strip_prefix("id=") {
            id = v.trim_matches('"');
        } else if let Some(v) = line.strip_prefix("id_like=") {
            id_like = v.trim_matches('"');
        }
    }

    let base = format!("{id} {id_like}");

    if base.contains("arch") || base.contains("manjaro") {
        "arch"
    } else if base.contains("fedora") || base.contains("rhel") || base.contains("centos") {
        "fedora"
    } else if base.contains("debian") || base.contains("ubuntu") || base.contains("linuxmint") {
        "debian"
    } else {
        "unknown"
    }
}

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
    let latest_release = get_latest_release().unwrap_or_else(|| VERSION.to_string());

    match installation_method(Path::new("/usr/bin/cmdcreate")) {
        InstallMethod::Aur => upgrade_aur(),
        InstallMethod::Dpkg => upgrade_deb(&latest_release),
        InstallMethod::Rpm => upgrade_rpm(&latest_release),
        InstallMethod::Other => interactive_upgrade(&latest_release),
    }
}

fn upgrade_aur() {
    run_shell_command(
        "rm -rf ~/cmdcreate; \
         git clone --branch cmdcreate --single-branch https://github.com/archlinux/aur.git ~/cmdcreate; \
         cd ~/cmdcreate; \
         makepkg -si",
    );
    delete_folder(&format!("{}/cmdcreate", VARS.home));
}

fn upgrade_deb(latest_release: &str) {
    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.deb \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.deb; \
         sudo dpkg -i /tmp/cmdcreate-{latest_release}-linux-x86_64.deb"
    ));
}

fn upgrade_rpm(latest_release: &str) {
    run_shell_command(&format!(
        "curl -L -o /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm \
         https://github.com/owen-debiasio/cmdcreate/releases/latest/download/cmdcreate-{latest_release}-linux-x86_64.rpm; \
         sudo rpm -Uvh /tmp/cmdcreate-{latest_release}-linux-x86_64.rpm"
    ));
}

fn upgrade_binary(latest_release: &str) {
    let file_to_download = format!("cmdcreate-{latest_release}-linux-bin");
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
        .find(|a| a.name == file_to_download)
    {
        let tmp_path = format!("/tmp/{}", asset.name);
        let out_path = "/usr/bin/cmdcreate";

        let mut resp = client
            .get(&asset.browser_download_url)
            .send()
            .expect("Failed to download binary");

        copy(
            &mut resp,
            &mut File::create(&tmp_path).expect("Failed to create temp file"),
        )
        .expect("Failed to copy binary");

        run_shell_command(&format!(
            "sudo chmod +x {tmp_path}; \
             sudo mv {tmp_path} {out_path}"
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

    run_shell_command(
        "rm -rf ~/.cache/cmdcreate && \
         git clone https://github.com/owen-debiasio/cmdcreate ~/.cache/cmdcreate",
    );

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

    println!("{green}Successfully built from source!{reset}");
}

fn interactive_upgrade(latest_release: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    ask_for_confirmation("Do you want to upgrade cmdcreate?");

    println!("\nSelect an upgrade method:\n");

    for (i, option) in [
        &format!("Upgrade through AUR {blue}(universal device compatibility){reset}"),
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

    let mut method_input = String::new();
    stdin().read_line(&mut method_input).unwrap();

    match method_input.trim() {
        "1" => upgrade_aur(),
        "2" => upgrade_deb(latest_release),
        "3" => upgrade_rpm(latest_release),
        "4" => upgrade_binary(latest_release),
        "5" => build_from_source(),
        "6" => error("Aborted.", ""),
        _ => error("Invalid selection.", ""),
    }
}

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

pub fn get_latest_release() -> Option<String> {
    get_latest_tag("owen-debiasio", "cmdcreate").ok()
}

pub fn check_for_updates() {
    println!("\nChecking for updates...");
    match get_latest_release() {
        Some(latest) if latest != VERSION => {
            println!(
                "{}Update available: {VERSION} -> {latest}{}",
                COLORS.green, COLORS.reset
            );
            ask_for_confirmation("Do you want to upgrade cmdcreate?");
            upgrade();
        }
        Some(_) => println!("Already up to date."),
        None => error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        ),
    }
}
