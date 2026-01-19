use std::{fs::read_dir, path::PathBuf};

use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::error,
    },
};

pub fn is_command_installed(cmd: &str) {
    log(
        &format!(
            "cmds/tools::is_command_installed(): Checking if command \"{cmd}\" is installed..."
        ),
        0,
    );

    if !path_exists(&format!("{}/{cmd}", PATHS.install_dir)) {
        error(&format!("Command \"{cmd}\" is not installed"), "");
    }

    log(
        &format!(
            "cmds/tools::is_command_installed(): Command \"{cmd}\" is installed, continuing..."
        ),
        0,
    );
}

pub fn get_installed_commands() -> Vec<PathBuf> {
    let (red, reset) = (COLORS.red, COLORS.reset);

    log(
        "cmds/tools::get_installed_commands(): Getting installed commands...",
        0,
    );

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
