/// Module for importing commands from backup files in cmdcreate
///
/// This module provides functionality to restore commands from a backup file
/// created by the export function. It processes the backup file, recreates
/// the commands in the system, and sets up proper permissions and symlinks.
///
/// The import process expects files in the format:
/// ```text
/// command_name, command_contents
/// ```
/// Where each line represents one command to be restored.
use crate::utils::{
    colors::COLORS,                              // Terminal color formatting
    fs::{read_file_to_string, write_to_file},    // File operations
    sys::{return_args, run_shell_command, VARS}, // System operations and variables
};

/// Imports commands from a backup file
///
/// This function:
/// 1. Reads the specified backup file
/// 2. Parses each line for command name and contents
/// 3. For each command:
///    - Creates the command file
///    - Sets executable permissions
///    - Creates system symlinks
///
/// # Usage
/// ```bash
/// cmdcreate import <backup_file>
/// ```
///
/// # File Format
/// Each line in the backup file should be in the format:
/// ```text
/// command_name, command_contents
/// ```
pub fn import() {
    // Terminal color codes
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);
    let args = return_args();

    // Ensure the user provided an input file
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}");
        return;
    }

    // Read the import file into a string
    let contents = read_file_to_string(&args[1]);

    // Exit if the file is empty or unreadable
    if contents.trim().is_empty() {
        println!("{yellow}Import file is empty or unreadable.{reset}");
        return;
    }

    // Process each line in the import file
    for line in contents.replace("[|", "|").lines() {
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();

        if line.trim().is_empty() && !parts.is_empty() {
            continue; // Split the line into command name and contents and skip empty lines
        }

        let name = parts[0]; // Command name
        let mut data = String::new(); // Command content
        let mut favorite = false; // Favorite flag

        // Parse additional parts (script content or "favorite" tag)
        for part in parts.iter().skip(1) {
            if *part == "favorite" {
                favorite = true; // Mark as favorite
            } else if !part.is_empty() {
                // Append line to the command data
                if data.is_empty() {
                    data.push('\n');
                }
                data.push_str(part);
            }
        }

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        // Path to store the command
        let script_path = format!("{}/.local/share/cmdcreate/files/{}", VARS.home, name);

        // Write the command data to the file
        write_to_file(&script_path, &data);

        // Make the file executable and link to /usr/bin
        run_shell_command(&format!(
            "chmod +x {script_path}; sudo ln -sf {script_path} /usr/bin/{name}"
        ));

        // Add command to favorites if flagged
        if favorite {
            write_to_file(
                &format!("{}/.local/share/cmdcreate/favorites", VARS.home),
                &format!("{name}\n"),
            );
        }
    }

    // Final success message
    println!("\n{green}Successfully imported commands.{reset}");
}
