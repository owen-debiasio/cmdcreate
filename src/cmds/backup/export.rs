use std::path::Path;

/// Module for exporting installed commands in cmdcreate
///
/// This module provides functionality to export all installed commands
/// to a backup file. The export includes both command names and their
/// contents, allowing for later restoration or transfer to another system.
///
/// The exported file uses a simple format where each line contains:
/// `command_name, command_contents`
use crate::cmds::tools::get_installed_commands; // Command listing functionality
use crate::utils::{
    colors::COLORS,                           // Terminal color formatting
    fs::{read_file_to_string, write_to_file}, // File operations
    sys::VARS,                                // System operations and variables
};

/// Exports all installed commands to a backup file
///
/// This function:
/// 1. Takes an output directory path from command arguments
/// 2. Retrieves all installed commands
/// 3. For each command:
///    - Reads its contents
///    - Writes name and contents to the export file
/// 4. Creates a single export.cmdcreate file containing all commands
///
/// # Usage
/// ```bash
/// cmdcreate export <output_directory>
/// ```
///
/// # Export Format
/// The export file contains one command per line in the format:
/// ```text
/// command_name, command_contents
/// ```
///
pub fn export(path: &str) {
    // Terminal color codes
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    // Construct the full path to the export file
    let export_file = Path::new(path).join("export.cmdcreate");

    // Iterate over all installed commands
    for script in get_installed_commands() {
        if let Some(stem) = script.file_stem() {
            let cmd = stem.to_string_lossy();

            // Read the command's file content
            let mut cmd_contents =
                read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home));

            // Escape any '|' characters in the command contents
            cmd_contents = cmd_contents.replace("|", "[|");

            // Format the line to include favorite marker if applicable
            let line =
                if read_file_to_string(&format!("{}/.local/share/cmdcreate/favorites", VARS.home))
                    .contains(cmd.as_ref())
                {
                    format!("{cmd} | {cmd_contents} | favorite\n")
                } else {
                    format!("{cmd} | {cmd_contents}\n")
                };

            // Write the command data to the export file
            write_to_file(export_file.to_str().unwrap(), &line);
        }
    }

    // Print final success message
    println!(
        "{green}Successfully exported commands to:{blue} \"{}\"{green}.{reset}",
        export_file.display()
    );
}
