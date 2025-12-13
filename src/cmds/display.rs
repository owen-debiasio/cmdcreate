/// Module for displaying command contents in cmdcreate
///
/// This module provides functionality to view the contents of existing custom
/// commands. It allows users to inspect the actual script or code that makes
/// up a command without having to navigate to the command file directly.
use crate::utils::{
    colors::COLORS,                   // Terminal color formatting
    fs::{read_file_to_string, PATHS}, // System operations and variables
};

/// Displays the contents of a specified command
///
/// This function:
/// 1. Validates the command name is provided
/// 2. Reads the command file contents
/// 3. Displays the contents with formatting
///
/// # Usage
/// ```bash
/// cmdcreate display <command_name>
/// ```
pub fn display(cmd: &str) {
    // Initialize color codes for terminal output formatting
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    // Get command line arguments and validate argument count

    // Read and display the command's contents with formatting
    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim()
    );
}
