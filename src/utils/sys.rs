use std::{
    env::{args, consts, var},
    path::Path,
    process::{Command, Stdio},
    sync::LazyLock,
};

use crate::{
    configs::load,
    utils::{fs::read_file_to_string, msgs::error},
};

pub struct Vars {
    pub shell: String,
    pub home: String,
}

pub static VARS: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
    home: var("HOME").unwrap_or_else(|_| "unknown".to_string()),
});

pub static ARCH: &str = consts::ARCH;

pub fn return_args() -> Vec<String> {
    args().skip(1).collect()
}

pub fn args_contains(arg: &str) -> bool {
    return_args().contains(&arg.to_string())
}

pub fn run_shell_command(cmd: &str) {
    let shell: &str = if args_contains("--force_system_shell") | args_contains("-F") {
        &VARS.shell
    } else {
        &load("sys", "shell", "sh")
    };

    if cmd.trim().is_empty() {
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
        Ok(_) => {}

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

pub fn get_distro_base() -> &'static str {
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
