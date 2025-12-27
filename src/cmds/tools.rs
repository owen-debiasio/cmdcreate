use std::{fs::read_dir, path::PathBuf};

use crate::utils::{colors::COLORS, fs::PATHS, msgs::error, sys::args_contains};

pub fn is_command_installed(cmd: &str) {
    let installed = get_installed_commands()
        .iter()
        .any(|script| script.file_stem().and_then(|s| s.to_str()) == Some(cmd));

    if !(installed || args_contains("-f") || args_contains("--force")) {
        error(&format!("Command \"{cmd}\" is not installed"), "");
    }
}

pub fn get_installed_commands() -> Vec<PathBuf> {
    let commands: Vec<PathBuf> = read_dir(&PATHS.install_dir)
        .unwrap_or_else(|_| {
            panic!(
                "{}Error: Failed to read install directory{}",
                COLORS.red, COLORS.reset
            )
        })
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if commands.is_empty() {
        error("No commands are installed.", "");
    }

    commands
}
