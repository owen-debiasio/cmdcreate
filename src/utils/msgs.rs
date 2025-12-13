/// Messaging Utilities for cmdcreate
///
/// This module provides functions to display usage instructions,
/// handle errors, and confirm user actions. These utilities ensure
/// consistent and user-friendly messaging across cmdcreate.
///
/// # Features
/// - Display usage instructions with color formatting
/// - Ask for user confirmation with optional force override
/// - Print errors and terminate the program
use std::process::exit;

use crate::utils::{colors::COLORS, sys::args_contains};
use crate::PROJ_VER;

/// Displays the full usage instructions for cmdcreate
///
/// Shows all commands, flags, arguments, offline options, and
/// a brief about section. Uses terminal colors for readability.
pub fn display_usage() {
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    let lines: Vec<String> = vec![
        format!("cmdcreate {PROJ_VER}"),
        format!(
            "Usage: cmdcreate {magenta}(flags){reset} [{blue}command{reset}, {cyan}argument{reset}] {yellow}<args> {magenta}(flags){reset}"
        ),
        "\nCommands:".into(),
        format!("  {blue}create{yellow}   <command>    <contents>{reset}  Create a command"),
        format!("  {blue}remove {yellow}  <command>{reset}                Remove a command"),
        format!(
            "  {blue}edit   {yellow}  <command>{reset}                Modify contents of a command"
        ),
        format!("  {blue}list{reset}                              Display installed commands"),
        format!(
            "  {blue}search {yellow}  <command>{reset}                Searches for matched command"
        ),
        format!(
            "  {blue}display {yellow} <command>{reset}                Display contents of a command"
        ),
        format!("  {blue}rename {yellow}  <command>    <new name>{reset}  Renames a command"),
        format!(
            "  {blue}favorite {yellow}<add/remove> <command>{reset}   Adds or removes a command from favorites"
        ),
        format!(
            "  {blue}repair{reset}                            Repairs installed commands if needed"
        ),
        "\n  Update:".into(),
        format!("    {blue}check{reset}                           Checks for updates"),
        format!("    {blue}update{reset}                          Updates cmdcreate"),
        "\n  Backup:".into(),
        format!(
            "    {blue}export{reset} {yellow}<output directory>{reset}       Exports your installed commands"
        ),
        format!(
            "    {blue}import{reset} {yellow}<file input>{reset}             Imports your exported commands"
        ),
        "\nArguments:".into(),
        format!("  {cyan}-v{reset},{cyan} --version {reset}                    Displays version"),
        format!(
            "  {cyan}-s{reset},{cyan} --supported_editors {reset}          Displays supported text editors"
        ),
        format!("  {cyan}-c{reset},{cyan} --changelog {reset}                  Displays changelog"),
        format!("  {cyan}-l{reset},{cyan} --license {reset}                    Displays license"),
        format!(
            "  {cyan}-d{reset},{cyan} --debugging {reset}                  Displays flags used for debugging"
        ),
        "\n  Offline:".into(),
        format!(
            "    {cyan}-g{reset},{cyan} --get_offline_files{reset}         Downloads files for offline use"
        ),
        format!(
            "    {cyan}-r{reset},{cyan} --remove_offline_files{reset}      Removes files for offline use"
        ),
        "\nAbout:".into(),
        "   Cmdcreate allows you to create custom commands for your Linux Terminal".into(),
        "   without needing to enter the same \"complex\" commands over and over".into(),
        "   (unless if your are lazy like me).".into(),
    ];

    // Print all lines to terminal
    for line in lines {
        println!("{line}");
    }
}

/// Prompts the user for a Yes/No confirmation before proceeding
///
/// # Arguments
/// * `q` - The question to display to the user
///
/// Honors the `--force` or `-f` flags to bypass confirmation
pub fn ask_for_confirmation(q: &str) {
    if !args_contains("--force") && !args_contains("-f") {
        println!("{q}\n(Y or N)");
        let mut confirm = String::new();
        std::io::stdin().read_line(&mut confirm).unwrap();
        if confirm.trim().to_lowercase() != "y" {
            println!("{}\nAborted.{}", COLORS.red, COLORS.reset);
            exit(0)
        }
    }
}

/// Prints an error message and exits the program
///
/// # Arguments
/// * `msg` - Custom error message
/// * `err` - Specific error details
///
/// Uses terminal red color for visibility
pub fn error(msg: &str, err: &str) {
    eprintln!("{}Error: {} {err}{}", COLORS.red, msg.trim(), COLORS.reset);
    exit(1)
}
