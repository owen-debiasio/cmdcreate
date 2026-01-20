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
    args().skip(1).any(|a| a == arg)
}

pub fn run_shell_command(cmd: &str) {
    let shell_owned;

    let shell: &str = if args_contains("--force_system_shell") || args_contains("-F") {
        &VARS.shell
    } else {
        shell_owned = load("sys", "shell", "sh");
        &shell_owned
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
            &format!(
                "utils/sys::run_shell_command(): Ran command \"{cmd}\" using shell \"{shell}\""
            ),
            0,
        ),
        Err(e) => error("Failed to run shell command:", &e.to_string()),
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
    log(
        "utils/sys::installation_method(): Detecting installation method...",
        0,
    );

    let Ok(path) = path.canonicalize() else {
        return InstallMethod::Other;
    };

    let Some(path_str) = path.to_str() else {
        return InstallMethod::Other;
    };

    match get_distro_base() {
        DistroBase::Arch => {
            if Command::new("pacman")
                .args(["-Qo", path_str])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as AUR/Pacman",
                    0,
                );
            } else {
                log(
                    "utils/sys::installation_method(): File not owned by pacman, assuming manual Arch install",
                    1,
                );
            }

            InstallMethod::Aur
        }

        DistroBase::Fedora => {
            log(
                "utils/sys::installation_method(): Checking if installation method is RPM...",
                0,
            );

            if Command::new("rpm")
                .args(["-qf", path_str])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as RPM",
                    0,
                );
            } else {
                log(
                    "utils/sys::installation_method(): File not owned by RPM, assuming manual Fedora install",
                    1,
                );
            }

            InstallMethod::Rpm
        }

        DistroBase::Debian => {
            log(
                "utils/sys::installation_method(): Checking if installation method is DPKG...",
                0,
            );

            if Command::new("dpkg-query")
                .args(["-S", path_str])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                log(
                    "utils/sys::installation_method(): Installation method detected as DPKG",
                    0,
                );
            } else {
                log(
                    "utils/sys::installation_method(): File not owned by DPKG, assuming manual Debian install",
                    1,
                );
            }

            InstallMethod::Dpkg
        }

        DistroBase::Unknown => {
            log(
                "utils/sys::installation_method(): Distro base unknown, classifying as Other",
                1,
            );

            InstallMethod::Other
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroBase {
    Arch,
    Fedora,
    Debian,
    Unknown,
}

pub fn get_distro_base() -> DistroBase {
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
            "utils/sys::get_distro_base(): Detected distro base as Arch",
            0,
        );

        DistroBase::Arch
    } else if base.contains("fedora") || base.contains("rhel") || base.contains("centos") {
        log(
            "utils/sys::get_distro_base(): Detected distro base as Fedora",
            0,
        );

        DistroBase::Fedora
    } else if base.contains("debian") || base.contains("ubuntu") || base.contains("linuxmint") {
        log(
            "utils/sys::get_distro_base(): Detected distro base as Debian",
            0,
        );

        DistroBase::Debian
    } else {
        log(
            "utils/sys::get_distro_base(): Unable to detect distro base",
            1,
        );

        DistroBase::Unknown
    }
}
