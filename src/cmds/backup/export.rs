/// Module for exporting installed commands in cmdcreate
///
/// This module provides functionality to export all installed commands
/// to a backup file. The export includes both command names and their
/// contents, allowing for later restoration or transfer to another system.
///
/// The exported file uses a simple format where each line contains:
/// `command_name, command_contents`
use crate::cmds::tools::retrieve_commands; // Command listing functionality
use crate::utils::{
    colors::COLORS,                           // Terminal color formatting
    fs::{read_file_to_string, write_to_file}, // File operations
    sys::{return_args, VARS},                 // System operations and variables
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
pub fn export() {
    use std::path::Path;

    // Terminal color codes
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    // Ensure the user provided an output directory
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}export {yellow}<output directory>{reset}");
        return;
    }

    // Construct the full path to the export file
    let export_file = Path::new(args.get(1).unwrap()).join("export.cmdcreate");

    // Read the favorites list for marking favorite commands
    let favorites = read_file_to_string(&format!("{}/.local/share/cmdcreate/favorites", VARS.home));

    // Iterate over all installed commands
    for script in retrieve_commands("installed") {
        if let Some(stem) = script.file_stem() {
            let cmd = stem.to_string_lossy();

            // Read the command's file content
            let mut cmd_contents =
                read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home));

            // Escape any '|' characters in the command contents
            cmd_contents = cmd_contents.replace("|", "[|");

            // Format the line to include favorite marker if applicable
            let line = if favorites.contains(cmd.as_ref()) {
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
