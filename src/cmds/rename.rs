/// Module for handling command renaming functionality in cmdcreate
///
/// This module provides functionality to rename existing commands while maintaining
/// their functionality and system integration. It handles both the command file
/// renaming and updating system symlinks.
use crate::{
    cmds::tools::{is_command_installed, retrieve_commands}, // Command management utilities
    utils::{
        colors::COLORS,         // Terminal color formatting
        sys::run_shell_command, // System operations
    },
};

/// Renames an existing command to a new name
///
/// This function:
/// 1. Validates command existence
/// 2. Renames the command file
/// 3. Updates system symlinks
/// 4. Maintains command functionality
///
/// # Usage
/// ```bash
/// cmdcreate rename <old_name> <new_name>
/// ```
pub fn rename(old: &str, new: &str) {
    // Initialize color codes for terminal output formatting
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    // Get list of installed commands and validate there are commands to rename
    let installed_scripts = retrieve_commands("installed");
    if installed_scripts.is_empty() {
        return;
    }

    // Validate base command exists
    is_command_installed(old);

    // Perform the rename operation:
    // 1. Rename the command file in cmdcreate's directory
    // 2. Move the system symlink to the new name
    // 3. Update the symlink to point to the new file
    run_shell_command(&format!(
        "
        mv ~/.local/share/cmdcreate/files/{old} ~/.local/share/cmdcreate/files/{new}; \
        sudo mv /usr/bin/{old} /usr/bin/{new}; \
        sudo ln -sf ~/.local/share/cmdcreate/files/{new} /usr/bin/{new}; \
        "
    ));

    // Confirm successful rename to user
    println!("{green}Successfully renamed command {blue}\"{old}\" to {blue}\"{new}\"{reset}")
}
