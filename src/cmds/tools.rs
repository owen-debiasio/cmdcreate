use std::{fs::read_dir, path::PathBuf};

use crate::utils::{
    colors::COLORS,
    fs::{PATHS, path_exists},
    io::error,
};

pub fn is_command_installed(cmd: &str) {
    if !path_exists(&format!("{}/{cmd}", PATHS.install_dir)) {
        error(&format!("Command \"{cmd}\" is not installed"), "");
    }
}

pub fn get_installed_commands() -> Vec<PathBuf> {
    let (red, reset) = (COLORS.red, COLORS.reset);

    let commands: Vec<PathBuf> = read_dir(&PATHS.install_dir)
        .unwrap_or_else(|_| panic!("{red}Error: Failed to read install directory{reset}",))
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if commands.is_empty() {
        error("No commands are installed.", "");
    }

    commands
}
