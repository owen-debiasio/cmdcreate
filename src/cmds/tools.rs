/// Internal utility module for shared command operations
///
/// This module provides core functionality used across other command modules,
/// including command validation, retrieval, and installation directory management.
/// It serves as the foundation for command operations throughout cmdcreate.
use std::{fs, path::PathBuf};

use crate::utils::{
    colors::COLORS,     // Terminal color formatting
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
    for script in retrieve_commands("installed") {
        if script.file_stem().unwrap_or_default().to_string_lossy() == *cmd {
            count += 1
        }
    }

    // Exit if command not found and not in force mode
    !(count == 0 && !(args_contains("-f") || args_contains("--force")))
}

/// Retrieves installed commands or the installation directory
///
/// This function serves two purposes based on the `val` parameter:
/// - When val="dir": Returns the installation directory path
/// - When val="installed": Returns paths to all installed commands
///
/// # Arguments
/// * `val` - Either "dir" or "installed" to specify return type
///
/// # Returns
/// * `Vec<PathBuf>` - Either:
///   - A vector containing just the install directory path
///   - A vector of paths to all installed command files
///   - An empty vector if no commands are installed
pub fn retrieve_commands(val: &str) -> Vec<PathBuf> {
    // Get the installation directory path
    let install_dir = dirs::home_dir()
        .unwrap_or_else(|| panic!("{}Error: Home dir not found{}", COLORS.red, COLORS.reset))
        .join(".local/share/cmdcreate/files");

    // Check if installation directory exists
    if !install_dir.exists() {
        error("No commands are installed.", "");
        return Vec::new();
    }

    // Return just the directory path if requested
    if val == "dir" {
        return vec![install_dir];
    }

    // Collect all command files from the installation directory
    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .unwrap_or_else(|_| {
            panic!(
                "{}Error: Failed to read install directory{}",
                COLORS.red, COLORS.reset
            )
        })
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Handle case where no commands are installed
    if installed_scripts.is_empty() {
        error("No commands are installed.", "");
    }

    // Return either the list of installed commands or an empty vector
    if val == "installed" {
        installed_scripts
    } else {
        Vec::new()
    }
}
