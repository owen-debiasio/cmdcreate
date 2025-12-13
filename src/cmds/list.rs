/// Module for displaying a list of installed commands in cmdcreate
///
/// This module provides functionality to view all currently installed custom
/// commands in the system. It shows the total count and names of all commands
/// that have been created using cmdcreate.
use crate::{
    cmds::tools::get_installed_commands, // Command retrieval utility
    utils::{
        colors::COLORS,
        fs::{read_file_to_string, PATHS},
    }, // Terminal color formatting
};

/// Lists all installed commands in the system
///
/// This function:
/// 1. Retrieves all installed commands
/// 2. Displays the total count
/// 3. Shows a formatted list of command names
///
/// # Usage
/// ```bash
/// cmdcreate list
/// ```
///
/// # Implementation Details
/// - Uses the `retrieve_commands` function to get a list of installed commands
/// - Formats the output using terminal color codes
/// - Marks favorite commands with a star (★)
pub fn list() {
    // Initialize color codes for terminal output formatting
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    // Get list of all installed commands
    let installed_scripts = get_installed_commands();

    // Display header with total command count
    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_scripts.len()
    );

    // Print each command name, extracting just the filename without path
    for script in installed_scripts {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();
        let favorites = read_file_to_string(&PATHS.favorites);

        // Check if command is marked as favorite
        if favorites.contains(name.to_string().as_str()) {
            // Prepend star to favorite command names
            println!("★ {name}");
            continue;
        }

        // Deal with normal command formatting
        if !favorites.is_empty() {
            println!("  {name}");
        } else {
            println!("{name}");
        }
    }
}
