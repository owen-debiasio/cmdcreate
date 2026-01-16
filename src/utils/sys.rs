use std::{
    env::{args, consts::ARCH as ARCHITECTURE, var},
    path::Path,
    process::{Command, Stdio},
    sync::LazyLock,
};

use crate::{
    configs::load,
    logger::log,
    utils::{fs::read_file_to_string, io::error},
};

pub struct Vars {
    pub shell: String,
    pub home: String,
}

pub static VARS: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
    home: var("HOME").unwrap_or_else(|_| "unknown".to_string()),
});

pub static ARCH: &str = ARCHITECTURE;

pub fn return_args() -> Vec<String> {
    // This piece of shit causes a stack overflow
    //log("utils/sys::return_args(): Retrieving args...", 0);

    args().skip(1).collect()
}

pub fn args_forced() -> bool {
    log(
        "utils/sys::args_forced(): Checking if args are forced...",
        0,
    );
    args_contains("--force") || args_contains("-f")
}

pub fn args_contains(arg: &str) -> bool {
    // Also a stack overlow causing piece of shit
    //log(
    //"utils/sys::args_contains(): Checking if args contain \"{arg}\"...",
    //0,
    //);

    return_args().contains(&arg.to_string())
}

pub fn run_shell_command(cmd: &str) {
    let shell: &str = if args_contains("--force_system_shell") | args_contains("-F") {
        &VARS.shell
    } else {
        &load("sys", "shell", "sh")
    };

    log(
        &format!("utils/sys::run_shell_command(): Using shell: \"{shell}\""),
        0,
    );

    if cmd.trim().is_empty() {
        log(
            "utils/sys::run_shell_command(): No command supplied, skipping...",
            0,
        );
        return;
    }

    match Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => log(
            &format!("utils/sys::run_shell_command(): Ran command {cmd} using shell {shell}"),
            0,
        ),

        Err(e) => {
            error("Failed to run shell command:", &e.to_string());
        }
    }
}

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

    log(
        "utils/sys::installation_method(): Detecting installation method...",
        0,
    );

    match get_distro_base() {
        "arch" => {
            log(
                "utils/sys::installation_method(): Checking if installation method is Pacman...",
                0,
            );

            if Command::new("pacman")
                .args(["-Qo", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as Pacman",
                    0,
                );

                return InstallMethod::Aur;
            }

            log(
                "utils/sys::installation_method(): Installation method not detected as Pacman... moving on...",
                0,
            );
        }

        "fedora" => {
            log(
                "utils/sys::installation_method(): Checking if installation method is DNF...",
                0,
            );

            if Command::new("rpm")
                .args(["-qf", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as DNF",
                    0,
                );

                return InstallMethod::Rpm;
            }

            log(
                "utils/sys::installation_method(): Installation method not detected as DNF... moving on...",
                0,
            );
        }

        "debian" => {
            log(
                "utils/sys::installation_method(): Checking if installation method is DPKG...",
                0,
            );

            if Command::new("dpkg-query")
                .args(["-S", path_str])
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as DPKG",
                    0,
                );

                return InstallMethod::Dpkg;
            }

            log(
                "utils/sys::installation_method(): Installation method not detected as DPKG... moving on...",
                0,
            );
        }

        _ => log(
            "utils/sys::installation_method(): Distro base not detected... Classifying distro as \"Other\"...",
            1,
        ),
    }

    InstallMethod::Other
}

pub fn get_distro_base() -> &'static str {
    log("utils/sys::get_distro_base(): Detecting distro base...", 0);

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
        log(
            "utils/sys::get_distro_base(): Detected distro base as \"Arch\"",
            0,
        );

        "arch"
    } else if base.contains("fedora") || base.contains("rhel") || base.contains("centos") {
        log(
            "utils/sys::get_distro_base(): Detected distro base as \"Fedora\"",
            0,
        );

        "fedora"
    } else if base.contains("debian") || base.contains("ubuntu") || base.contains("linuxmint") {
        log(
            "utils/sys::get_distro_base(): Detected distro base as \"Debian\"",
            0,
        );

        "debian"
    } else {
        log(
            "utils/sys::get_distro_base(): Unable to detect distro base, going as \"unknown\"",
            1,
        );

        "unknown"
    }
}
