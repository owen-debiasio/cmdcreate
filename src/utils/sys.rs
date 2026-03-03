// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Owen Debiasio
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    env::{args, consts::ARCH as ARCHITECTURE, var},
    process::{Command, Stdio},
    sync::LazyLock,
};

use rustix::process::geteuid;

use crate::{
    configs::load,
    logger::log,
    utils::{
        fs::read_file_to_string,
        io::{TestError, error},
    },
};

pub fn is_root() -> bool {
    geteuid().as_raw() == 0
}

pub struct Vars {
    pub shell: String,
    pub home: String,
    pub editor: String,
}

pub static VARS: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_owned()),
    home: var("HOME").unwrap_or_else(|_| "unknown".to_owned()),
    editor: var("EDITOR").unwrap_or_else(|_| "auto".to_owned()),
});

pub static ARCH: &str = ARCHITECTURE;

pub fn return_args() -> Vec<String> {
    let mut supplied_args = Vec::new();

    for arg in args().skip(1) {
        if arg.starts_with('-') && !arg.starts_with("--") && arg.len() > 2 {
            for ch in arg.chars().skip(1) {
                supplied_args.push(format!("-{ch}"));
            }

            continue;
        }

        supplied_args.push(arg);
    }

    supplied_args
}

pub fn args_forced() -> bool {
    args_contains("--force") || args_contains("-f")
}

pub fn args_contains(arg: &str) -> bool {
    return_args().iter().any(|a| a == arg)
}

pub fn run_shell_command_result(cmd: &str) -> Result<(), TestError> {
    if cmd.trim().is_empty() {
        return Ok(());
    }

    let shell_path = load("sys", "shell", "sh");
    let mut command = Command::new(&shell_path);
    command.arg("-c").arg(cmd);

    if cfg!(test) {
        let output = command
            .stdin(Stdio::null())
            .output()
            .map_err(|e| TestError(format!("Failed to execute {shell_path}: {e}")))?;

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(TestError(format!("Command failed: {stderr}")))
        }
    } else {
        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
        match command.status() {
            Ok(status) if status.success() => Ok(()),
            Ok(status) => Err(TestError(format!("Command failed with: {status}"))),
            Err(e) => Err(TestError(e.to_string())),
        }
    }
}

pub fn run_shell_command(cmd: &str) {
    if let Err(e) = run_shell_command_result(cmd) {
        error("Failed to run shell command:", &e.to_string());
    }
}

pub fn arch_is_supported() -> bool {
    ARCH == "x86_64"
}

pub fn cpu_arch_check(err_reason: &str) {
    log(
        "utils/sys::cpu_arch_check(): Double checking if CPU arch. is supported...",
        0,
    );

    if !arch_is_supported() {
        error(err_reason, "")
    }

    log(
        "utils/sys::cpu_arch_check(): CPU arch. is supported... Continuing action...",
        0,
    );
}

#[derive(Debug)]
pub enum InstallMethod {
    Aur,
    Dpkg,
    Other,
    Rpm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroBase {
    Arch,
    Debian,
    Fedora,
    Unknown,
}

pub fn get_distro_base() -> DistroBase {
    let (mut id, mut id_like) = ("", "");

    let os_release = read_file_to_string("/etc/os-release").to_lowercase();

    for line in os_release.lines() {
        if let Some(v) = line.strip_prefix("id=") {
            id = v.trim_matches('"');
        } else if let Some(v) = line.strip_prefix("id_like=") {
            id_like = v.trim_matches('"');
        }
    }

    let base = format!("{id} {id_like}");

    if base.contains("arch") || base.contains("manjaro") {
        DistroBase::Arch
    } else if base.contains("fedora") || base.contains("rhel") || base.contains("centos") {
        DistroBase::Fedora
    } else if base.contains("debian") || base.contains("ubuntu") || base.contains("linuxmint") {
        DistroBase::Debian
    } else {
        DistroBase::Unknown
    }
}

pub fn installation_method() -> InstallMethod {
    match get_distro_base() {
        DistroBase::Arch => InstallMethod::Aur,
        DistroBase::Fedora => InstallMethod::Rpm,
        DistroBase::Debian => InstallMethod::Dpkg,
        DistroBase::Unknown => InstallMethod::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::io::error_result;

    #[test]
    fn error_returns_err() {
        let result: Result<(), _> = error_result("nope");
        assert!(result.is_err());
    }

    #[test]
    fn error_message_matches() {
        assert_eq!(error_result::<()>("bad").unwrap_err().to_string(), "bad");
    }

    #[test]
    fn run_shell_command_result_fails_on_invalid_command() {
        assert!(
            run_shell_command_result("definitely_not_a_real_cmd").is_err(),
            "Expected error when running a non-existent command"
        );
    }

    #[test]
    fn distro_detection_returns_known_or_unknown() {
        let base = get_distro_base();
        assert!(
            base == DistroBase::Arch
                || base == DistroBase::Fedora
                || base == DistroBase::Debian
                || base == DistroBase::Unknown
        );
    }

    #[test]
    fn vars_are_initialized() {
        assert!(!VARS.home.is_empty());
        assert!(!VARS.shell.is_empty());
        assert!(!VARS.editor.is_empty());
    }
}
