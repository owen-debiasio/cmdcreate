/// cmdcreate: A command-line tool for managing custom shell commands
///
/// This tool allows users to create, edit, list, search, and manage custom shell commands
/// across their system. It provides features for command management, backup/restore,
/// and system integration.
///
/// # Features
/// - Command Management: Create, edit, remove, list, search, etc
/// - Backup & Restore: Import/export command configurations
/// - System Integration: Supports multiple editors and shell environments
/// - Update Management: Built-in version checking and updating
mod cmds;
mod utils;

use crate::{
    cmds::{
        backup::*,   // Handles command backup and restore functionality
        edit::*,     // Command editing operations
        upgrader::*, // Update checking and installation
        *,           // Everything else
    },
    utils::{colors::*, fs::*, msgs::*, sys::*}, // Utility modules for colors, filesystem ops, and messages
};

/// Current version of the project
pub const VERSION: &str = "v0.8.5";

/// Main entry point for the cmdcreate application
///
/// Processes command-line arguments and routes them to the appropriate
/// command handlers. Supports various operations including:
/// - Command management (create, remove, edit, list, search, display, rename)
/// - System operations (version check, updates)
/// - Backup operations (import, export)
/// - Information display (version, supported editors, license, changelog)
fn main() {
    // Initialize color codes for terminal output
    let (magenta, green, blue, yellow, reset) = (
        COLORS.magenta,
        COLORS.green,
        COLORS.blue,
        COLORS.yellow,
        COLORS.reset,
    );

    // Get command line arguments
    let args = return_args();
    if args.is_empty() {
        utils::msgs::display_usage();
        return;
    }

    // Disables Ctrl-C behavior
    ctrlc_enabled(false);

    // Pre-download license or changelog files if requested
    if args[0].starts_with("-l")
        || args[0].starts_with("--l")
        || args[0].starts_with("-c")
        || args[0].starts_with("--c")
    {
        get_files();
    }

    // Match the first argument to determine which operation to perform
    match args[0].as_str() {
        // Command Management Operations

        // Create a new command
        "create" => {
            if args.len() < 3 {
                println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}");
                return;
            }

            create::create(&args[1], &args[2], true)
        }

        // Remove an existing command
        "remove" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}");
                return;
            }

            remove::remove(&args[1])
        }

        // Edit an existing command
        "edit" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}edit {yellow}<command>{reset}");
                return;
            }

            edit(&args[1])
        }

        // Search for specific commands
        "search" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}");
                return;
            }

            search::search(&args[1])
        }

        // Display details of a command
        "display" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}");
                return;
            }

            display::display(&args[1])
        }

        // Rename an existing command
        "rename" => {
            if args.len() < 3 {
                println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}");
                return;
            }

            rename::rename(&args[1], &args[2])
        }

        // Add command to favorites
        "favorite" => {
            if args.len() < 3 || !["add", "remove"].contains(&args[1].as_str()) {
                println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}");
                return;
            }

            favorite::favorite(&args[1], &args[2])
        }

        "repair" => repair::repair(), // Repair a command
        "list" => list::list(),       // List all available commands

        // System Operations
        "check" => check_for_updates(), // Check for available updates
        "update" => upgrade(),          // Perform system upgrade

        // Backup Operations
        //
        // Import commands from backup
        "import" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}");
                return;
            }
            import::import(&args[1])
        }

        // Export commands to backup
        "export" => {
            if args.len() < 2 {
                println!("Usage:\ncmdcreate {blue}export {yellow}<output directory>{reset}");
                return;
            }
            export::export(&args[1])
        }

        // Information Display Arguments

        // Display version information
        "--version" | "-v" => println!(
            "cmdcreate {VERSION}\n(C) 2025 Owen Debiasio; distributed under the MIT License"
        ),

        // Display list of supported text editors
        "--supported_editors" | "-s" => {
            println!("Current supported editors:\n");
            for option in SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        // Download offline documentation files
        "--get_offline_files" | "-g" => {
            println!("Downloading offline files...");
            get_files();
            println!("{green}Files downloaded successfully.{reset}");
        }

        // Remove installed offline documentation
        "--remove_offline_files" | "-r" => {
            delete_file(&PATHS.changelog);
            delete_file(&PATHS.license);
            println!("Files removed successfully.");
        }

        // Display license information
        "--license" | "-l" => {
            println!("{}", read_file_to_string(&PATHS.license));
        }

        "--changelog" | "-c" => {
            // Display changelog
            println!("{}", read_file_to_string(&PATHS.changelog).trim());
        }

        "--debugging" | "-d" => {
            // Display debugging options and flags
            let lines: Vec<String> = vec![
                format!("Usage: cmdcreate {magenta}(flags){reset} [run] {magenta}(flags){reset}"),
                format!(
                    "  {magenta}-F{reset},{magenta} --force_system_shell{reset}    Forces system shell to be used when running commands"
                ),
                format!(
                    "  {magenta}-f{reset},{magenta} --force{reset}                 Skips confirmation for an action"
                ),
            ];

            for line in lines {
                println!("{line}")
            }
        }

        // Handle invalid inputs
        _ => {
            if args[0].starts_with("-") {
                // Report invalid argument error
                error("Invalid argument:", &args[0]);
            }

            // Report invalid command error
            error("Invalid command:", &args[0])
        }
    }

    // Re-enable Ctrl-C behavior
    ctrlc_enabled(true);
}
