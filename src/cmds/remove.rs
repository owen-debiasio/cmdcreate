/// Module for handling command removal functionality in cmdcreate
///
/// This module provides functionality to safely remove custom commands
/// that were previously installed using cmdcreate. It includes safety
/// checks and user confirmation before removal.
use crate::{
    cmds::tools::{get_installed_commands, is_command_installed}, // Command validation tools
    utils::{
        colors::COLORS,                                           // Terminal color formatting
        fs::{delete_file, read_file_to_string, remove_from_file}, // File system operations
        msgs::ask_for_confirmation,                               // User interaction
        sys::{run_shell_command, VARS},                           // System operations and variables
    },
};

/// Removes a specified command from the system
///
/// This function:
/// 1. Validates the command exists
/// 2. Asks for user confirmation
/// 3. Removes both the command file and system symlink
///
/// # Usage
/// ```bash
/// cmdcreate remove <command_name>
/// ```
pub fn remove(command: &str) {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    // Check if there are any installed commands
    if get_installed_commands().is_empty() {
        return; // Exit if no commands are installed
    }

    // Verify that the specified command exists
    is_command_installed(command);

    // Request user confirmation before deletion
    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
    ));

    // Remove the command file from cmdcreate's storage
    delete_file(&format!(
        "{}/.local/share/cmdcreate/files/{command}",
        VARS.home
    ));

    // Remove the command from the list of favorites
    let path = format!("{}/.local/share/cmdcreate/favorites", VARS.home);
    if read_file_to_string(&path).contains(command) {
        remove_from_file(&path, command);
    }

    // Remove the system-wide command symlink
    run_shell_command(&format!("sudo rm -f /usr/bin/{command}"));

    // Confirm successful removal to user
    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
