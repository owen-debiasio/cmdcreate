/// cmdcreate: A command-line tool for managing custom shell commands
///
/// This tool allows users to create, edit, list, search, and manage custom shell commands
/// across their system. It provides features for command management, backup/restore,
/// and system integration.
///
/// # Features
/// - Command Management: Create, edit, remove, list, search commands
/// - Backup & Restore: Import/export command configurations
/// - System Integration: Supports multiple editors and shell environments
/// - Update Management: Built-in version checking and updating
mod cmds;
mod utils;

use crate::utils::sys::*;
use crate::{
    cmds::{
        backup::{export, import}, // Handles command backup and restore functionality
        edit::*,                  // Command editing operations
        upgrader::*,              // Update checking and installation
        *,                        // All other command operations
    },
    utils::{colors::*, fs::*, msgs::*}, // Utility modules for colors, filesystem ops, and messages
};

/// Current version of the project
pub static PROJ_VER: &str = "v0.7.8";

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

    // Pre-download license or changelog files if requested
    if args[0].starts_with("-l")
        || args[0].starts_with("--l")
        || args[0].starts_with("-c")
        || args[0].starts_with("--c")
    {
        run_shell_command(
            "
            curl -sSo ~/.local/share/cmdcreate/LICENSE https://raw.githubusercontent.com/owen-debiasio/cmdcreate/master/LICENSE; \
            curl -sSo ~/.local/share/cmdcreate/changes.md https://raw.githubusercontent.com/owen-debiasio/cmdcreate/master/changes.md
            "
        );
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

            edit::edit(&args[1])
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
        "import" => import::import(), // Import commands from backup
        "export" => export::export(), // Export commands to backup

        // Information Display Arguments
        "--version" | "-v" => println!("cmdcreate {PROJ_VER}"), // Display version information

        "--supported_editors" | "-s" => {
            // Display list of supported text editors
            println!("Current supported editors:\n");
            for option in SUPPORTED_EDITORS {
                println!("{option}")
            }
        }

        "--get_offline_files" | "-g" => {
            // Download offline documentation files
            println!("Downloading offline files...");
            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            // Remove installed offline documentation
            delete_file(&format!("{}/.local/share/cmdcreate/changes.md", VARS.home));
            delete_file(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home));
            println!("Files removed successfully.");
        }

        "--license" | "-l" => {
            // Display license information
            println!(
                "{}",
                read_file_to_string(&format!("{}/.local/share/cmdcreate/LICENSE", VARS.home))
            );
        }

        "--changelog" | "-c" => {
            // Display changelog
            println!(
                "{}",
                read_file_to_string(&format!("{}/.local/share/cmdcreate/changes.md", VARS.home))
                    .trim()
            );
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
}
