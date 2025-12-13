/// Module for searching installed commands in cmdcreate
///
/// This module provides functionality to search through installed commands
/// by name, allowing users to find specific commands or groups of related
/// commands based on partial name matches.
use crate::{
    cmds::tools::get_installed_commands, // Command retrieval utility
    utils::{
        colors::COLORS, // Terminal color formatting
        msgs::error,    // Error message handling
    },
};

/// Searches for installed commands by name
///
/// This function:
/// 1. Takes a search term from command line arguments
/// 2. Searches through all installed commands
/// 3. Displays matching command names
/// 4. Shows error if no matches found
///
/// # Usage
/// ```bash
/// cmdcreate search <search_term>
/// ```
pub fn search(cmd: &str) {
    // Initialize color codes for terminal output formatting
    let (yellow, reset) = (COLORS.yellow, COLORS.reset);

    // Extract search term and get list of installed commands
    let installed_scripts = get_installed_commands();

    // Search through installed commands and count matches
    let mut count = 0;
    for script in installed_scripts {
        // Extract command name from file path
        let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

        // Print matching command names
        if file_stem.contains(cmd) {
            println!("{file_stem}");
            count += 1
        }
    }

    // Display error if no matches found
    if count == 0 {
        error(
            "No installed commands contain:",
            &format!("{yellow}\"{cmd}\"{reset}"),
        )
    }
}
