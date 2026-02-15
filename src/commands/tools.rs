use std::{fs::read_dir, path::PathBuf};

use crate::{
    logger::log,
    utils::{colors::COLORS, fs::path_exists, io::error},
};

pub fn command_is_installed(cmd: &str) -> bool {
    if path_exists(&format!("/usr/local/bin/{cmd}")) {
        log(
            &format!(
                "commands/tools::command_is_installed(): Command \"{cmd}\" is installed... Continuing..."
            ),
            0,
        );

        return true;
    }

    error(&format!("Command \"{cmd}\" is not installed"), "");
}

pub fn get_installed_commands() -> Vec<PathBuf> {
    let (red, reset) = (COLORS.red, COLORS.reset);

    log(
        "commands/tools::get_installed_commands(): Getting installed commands...",
        0,
    );

    let commands: Vec<PathBuf> = read_dir("/usr/local/bin/")
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
