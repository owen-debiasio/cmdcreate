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

use crate::{
    configs::load_configuration,
    logger::{Severity, log},
    utils::{fs::read_file_to_string, io::error},
};
use rustix::process::geteuid;
use std::{
    env::{args, consts::ARCH as ARCHITECTURE, var},
    process::{Command, Stdio},
    sync::{LazyLock, OnceLock},
};

pub fn running_as_root() -> bool {
    // 0 is root, if it returns anything else, cmdcreate won't run
    geteuid().as_raw() == 0
}

pub fn root_requirement_is_bypassed() -> bool {
    // Either of these flags allow root bypass
    args_contains("-b") || args_contains("--bypass-root")
}

pub struct Vars {
    pub shell: String,
    pub text_editor: String,
}

pub static ENVIRONMENT_VARIABLES: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_owned()),
    text_editor: var("EDITOR").unwrap_or_else(|_| "auto".to_owned()),
});

pub static ARCH: &str = ARCHITECTURE;

pub fn args_contains(argument: &str) -> bool {
    static ARGS: OnceLock<Vec<String>> = OnceLock::new();

    let processed_args = ARGS.get_or_init(return_args);

    processed_args.iter().any(|arg| arg == argument)
}

pub fn return_args() -> Vec<String> {
    let mut supplied_argument_vector = Vec::new();

    let mut supplied_argument_is_actually_an_argument;

    for supplied_argument in args().skip(1) {
        supplied_argument_is_actually_an_argument = supplied_argument.starts_with('-');

        if supplied_argument_is_actually_an_argument && supplied_argument.len() > 2 {
            for character in supplied_argument.chars().skip(1) {
                supplied_argument_vector.push(format!("-{character}"));
            }
        } else {
            supplied_argument_vector.push(supplied_argument);
        }
    }

    supplied_argument_vector
}

pub fn arguments_force_actions() -> bool {
    // If either are applied, returns true
    args_contains("--force") || args_contains("-f")
}

pub fn system_command_is_installed(command_to_check: &str) -> bool {
    Command::new("which")
        .arg(command_to_check)
        .output()
        .map(|output_status| output_status.status.success())
        .unwrap_or(false)
}

pub fn run_shell_command(command: &str) {
    let shell: &str = &load_configuration("sys", "shell", &ENVIRONMENT_VARIABLES.shell);

    if command.trim().is_empty() {
        return;
    }

    // "set -e" is included to make all commands exit on fail
    let command_to_run: &str = &format!(
        "set -e && \
        {command}"
    );

    match Command::new(shell)
        .arg("-c")
        .arg(command_to_run)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {}
        Err(error_message) => {
            error("Failed to run shell command:", &error_message.to_string());
        }
    }
}

pub fn arch_is_supported() -> bool {
    ARCH == "x86_64"
}

pub fn cpu_arch_check(error_reason: &str) {
    log(
        "utils/sys::cpu_arch_check(): Double checking if CPU arch. is supported...",
        Severity::Normal,
    );

    if !arch_is_supported() {
        error(error_reason, "")
    }

    log(
        "utils/sys::cpu_arch_check(): CPU arch. is supported... Continuing action...",
        Severity::Normal,
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

/// This function is kind of fucked so bear with me here
pub fn get_distro_base() -> DistroBase {
    let (mut distro_id, mut distro_id_alt) = ("", "");

    let os_release = read_file_to_string("/etc/os-release").to_lowercase();

    for line in os_release.lines() {
        if let Some(distro_release_version) = line.strip_prefix("id=") {
            distro_id = distro_release_version.trim_matches('"');
        } else if let Some(distro_release_version_alt) = line.strip_prefix("id_like=") {
            distro_id_alt = distro_release_version_alt.trim_matches('"');
        }
    }

    let distro_base = format!("{distro_id} {distro_id_alt}");

    if distro_base.contains("arch") || distro_base.contains("manjaro") {
        DistroBase::Arch
    } else if distro_base.contains("fedora")
        || distro_base.contains("rhel")
        || distro_base.contains("centos")
    {
        DistroBase::Fedora
    } else if distro_base.contains("debian")
        || distro_base.contains("ubuntu")
        || distro_base.contains("linuxmint")
    {
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

    #[test]
    fn distro_detection_returns_known_or_unknown() {
        let distro_base = get_distro_base();
        assert!(
            distro_base == DistroBase::Arch
                || distro_base == DistroBase::Fedora
                || distro_base == DistroBase::Debian
                || distro_base == DistroBase::Unknown
        );
    }

    #[test]
    fn vars_are_initialized() {
        assert!(!ENVIRONMENT_VARIABLES.shell.is_empty());
        assert!(!ENVIRONMENT_VARIABLES.text_editor.is_empty());
    }
}
