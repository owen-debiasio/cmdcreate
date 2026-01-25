use std::{
    env::{args, consts::ARCH as ARCHITECTURE, var},
    path::Path,
    process::{Command, Stdio},
    sync::LazyLock,
};

use crate::{
    configs::load,
    utils::{
        fs::read_file_to_string,
        io::{TestError, error},
    },
};

pub struct Vars {
    pub shell: String,
    pub home: String,
}

pub static VARS: LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_owned()),
    home: var("HOME").unwrap_or_else(|_| "unknown".to_owned()),
});

pub static ARCH: &str = ARCHITECTURE;

pub fn return_args() -> Vec<String> {
    args().skip(1).collect()
}

pub fn args_forced() -> bool {
    args_contains("--force") || args_contains("-f")
}

pub fn args_contains(arg: &str) -> bool {
    args().skip(1).any(|a| a == arg)
}

pub fn run_shell_command_result(cmd: &str) -> Result<(), TestError> {
    let shell = if args_contains("--force_system_shell") || args_contains("-F") {
        VARS.shell.clone()
    } else {
        load("sys", "shell", "sh")
    };

    if cmd.trim().is_empty() {
        return Ok(());
    }

    match Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(TestError(format!("Command exited with code {status}"))),
        Err(e) => Err(TestError(e.to_string())),
    }
}

pub fn run_shell_command(cmd: &str) {
    if let Err(e) = run_shell_command_result(cmd) {
        error("Failed to run shell command:", &e.to_string());
    }
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
    let content = read_file_to_string("/etc/os-release").to_lowercase();
    let (mut id, mut id_like) = ("", "");

    for line in content.lines() {
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

pub fn installation_method(path: &Path) -> InstallMethod {
    let Ok(path) = path.canonicalize() else {
        return InstallMethod::Other;
    };

    let Some(_path_str) = path.to_str() else {
        return InstallMethod::Other;
    };

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
    }
}
