/// Internal utility module for shared command operations
///
/// This module provides core functionality used across other command modules,
/// including command validation, retrieval, and installation directory management.
/// It serves as the foundation for command operations throughout cmdcreate.
use std::{fs::read_dir, path::PathBuf};

use crate::utils::{
    colors::COLORS,     // Terminal color formatting
    fs::*,              // File system utilities
    msgs::error,        // Error message handling
    sys::args_contains, // Command line argument checking
};

/// Checks if a specific command is installed in the system
///
/// This function verifies whether a given command name exists in the
/// installed commands directory. It will exit the program if the command
/// is not found, unless the force flag (-f, --force) is used.
///
/// # Arguments
/// * `cmd` - The name of the command to check
///
/// # Exit
/// Exits with status 0 if command is not found and force flag is not set
pub fn is_command_installed(cmd: &str) -> bool {
    // Count occurrences of the command in installed commands
    let mut count: i32 = 0;
    for script in get_installed_commands() {
        if script.file_stem().unwrap_or_default().to_string_lossy() == *cmd {
            count += 1
        }
    }

    // Exit if command not found and not in force mode
    !(count == 0 && !(args_contains("-f") || args_contains("--force")))
}

/// Retrieves installed commands
///
/// This function returns paths to all installed command files
/// located in the installation directory.
///
/// # Returns
/// * `Vec<PathBuf>` -
///   - A vector of paths to all installed command files
///   - An empty vector if no commands are installed
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
