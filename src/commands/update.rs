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
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::{error::Error, fs::File, io::copy, path::Path, process::exit, time::Duration};

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

fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("cmdcreate-upgrader")
        .build()
        .expect("Failed to build HTTP client")
}

pub fn update() {
    let (green, blue, red, reset) = (COLORS.green, COLORS.blue, COLORS.red, COLORS.reset);

    log("cmds/update::update(): Initializing upgrade process...", 0);

    let latest_release = &get_latest_release().unwrap_or_else(|| VERSION.to_owned());

    log(
        "cmds/update::update(): Determining installation method...",
        0,
    );

    log("cmds/update::update(): Getting installation paths...", 0);

    log(
        "cmds/update::interactive_upgrade(): Requesting permission to upgrade...",
        0,
    );

    ask_for_confirmation("\nDo you want to upgrade cmdcreate?");

    log(
        "cmds/update::interactive_upgrade(): Continuing with upgrade...",
        0,
    );

    match installation_method(Option::from(get_install_path())) {
        InstallMethod::Aur => {
            log("cmds/update::update(): Arch Linux detected...", 0);

            if !args_forced()
                && input(&format!(
                "\n{blue}Arch Linux{reset}-based system detected. Would you like to install through the {blue}AUR{reset}?\n({green}Y{reset} or {red}N{reset})"
            ))
                .trim()
                .eq_ignore_ascii_case("n")
            {
                println!();
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
            log("cmds/update::update(): Debian/Ubuntu detected...", 0);

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

        InstallMethod::Rpm => {
            log("cmds/update::update(): Fedora detected...", 0);

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
        "git clone https://aur.archlinux.org/{pkg}.git /tmp/{pkg} && \
         cd /tmp/{pkg} && \
         makepkg -si --noconfirm"
    ));

    delete_folder(&format!("{}/tmp/{pkg}", VARS.home));

    log("cmds/update::update_aur(): Update completed.", 0);

    println!("{green}Update complete!{reset}");
}

fn upgrade_deb(latest_release: &str) {
    let (green, reset) = (COLORS.green, COLORS.reset);

    log("cmds/update::update_deb(): Installing .deb package...", 0);

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

    log("cmds/update::update_rpm(): Installing .rpm package...", 0);

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

    let cache_dir = format!("{}/.cache/cmdcreate", VARS.home);
    delete_folder(&cache_dir);

    log(
        "cmds/update::build_from_source(): Retrieving distro base...",
        0,
    );

    log(
        "cmds/update::build_from_source(): Installing build dependencies...",
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
            exit(1);
        }
    });

    log("cmds/update::build_from_source(): Cloning repository...", 0);

    run_shell_command(&format!(
        "git clone --depth=1 https://github.com/owen-debiasio/cmdcreate {cache_dir}",
    ));

    log(
        "cmds/update::build_from_source(): Building and installing package...",
        0,
    );

    run_shell_command(&format!(
        "set -e && \
         cd \"{cache_dir}\" && \
         cargo build --release && \
         sudo install -Dm755 target/release/cmdcreate /usr/bin/cmdcreate",
    ));

    log("cmds/update::build_from_source(): Update complete...", 0);

    println!("\n{green}Update complete!{reset}");
}

fn interactive_upgrade(latest_release: &str) {
    let (blue, green, red, reset) = (COLORS.blue, COLORS.green, COLORS.red, COLORS.reset);

    let latest_commit: &str = &get_latest_commit("owen-debiasio", "cmdcreate", "main");

    log(
        "cmds/update::interactive_upgrade(): Initializing interactive upgrade...",
        0,
    );

    log(
        "cmds/update::interactive_upgrade(): Providing upgrade options...",
        0,
    );

    println!("\nSelect an upgrade method:\n");

    for (i, option) in [
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
        .to_owned();

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

    let current = VERSION;
    let Some(latest) = get_latest_release() else {
        error(
            "Failed to check for updates. Ensure you are connected to the internet.",
            "",
        );
        return;
    };

    log(
        &format!(
            "cmds/update::check_for_updates(): Comparing versions \"{current} (Current)\" to \"{latest}\" (Latest)..."
        ),
        0,
    );

    let parse_version = |v: &str| -> (u32, u32, u32) {
        let nums: Vec<u32> = v
            .trim_start_matches('v')
            .split('.')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        (
            *nums.first().unwrap_or(&0),
            *nums.get(1).unwrap_or(&0),
            *nums.get(2).unwrap_or(&0),
        )
    };

    match parse_version(current).cmp(&parse_version(&latest)) {
        Ordering::Less => {
            log(
                &format!(
                    "cmds/update::check_for_updates(): Found available update from \"{current}\" to \"{latest}\"..."
                ),
                0,
            );
            println!("{green}\nUpdate available: {current} -> {latest}{reset}");

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
        Ordering::Greater => {
            log(
                "cmds/update::check_for_updates(): Current version is newer than the latest release.",
                1,
            );
            println!(
                "You are running a newer version {}({current}){reset} than the latest release {green}({latest}){reset}.\
                \nAssuming it's a development build.",
                COLORS.blue,
            );
        }
        Ordering::Equal => {
            log("cmds/update::check_for_updates(): No updates available.", 1);
            println!("Already up to date.");
        }
    }
}

pub fn get_latest_commit(owner: &str, repo: &str, branch: &str) -> String {
    log(
        "commands/update::get_latest_commit(): Retrieving latest commit...",
        0,
    );

    let commit: Value = http_client()
        .get(format!(
            "https://api.github.com/repos/{owner}/{repo}/commits/{branch}"
        ))
        .send()
        .expect("request failed")
        .json()
        .expect("invalid json");

    let commit = commit["sha"]
        .as_str()
        .expect("missing sha")
        .chars()
        .take(7)
        .collect::<String>();

    log(
        &format!("commands/update::get_latest_commit(): Retrieved latest commit: \"{commit}\""),
        0,
    );

    commit
}
