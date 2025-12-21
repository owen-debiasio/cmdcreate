use std::{fs::read_dir, path::PathBuf};

use crate::utils::{colors::COLORS, fs::PATHS, msgs::error, sys::args_contains};

pub fn is_command_installed(cmd: &str) -> bool {
    let mut count: i32 = 0;
    for script in get_installed_commands() {
        if script.file_stem().unwrap_or_default().to_string_lossy() == *cmd {
            count += 1;
        }
    }

    !(count == 0 && !(args_contains("-f") || args_contains("--force")))
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
