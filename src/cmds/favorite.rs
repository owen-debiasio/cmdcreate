//! Favorite Command Management
//!
//! This module handles marking commands as “favorites” so users can
//! easily keep track of their most-used custom commands.
//!
//! # Features
//! - `favorite add <command>` — Add a command to the favorites list
//! - `favorite remove <command>` — Remove a command from the favorites list
//!
//! Favorites are stored in:
//! `~/.local/share/cmdcreate/favorites`

use std::fs::read_to_string;

use crate::{
    cmds::tools::is_command_installed,
    utils::{
        colors::COLORS,
        fs::{create_file, remove_from_file, write_to_file},
        sys::VARS,
    },
};

/// Entry point for the `favorite` command.
///
/// Parses arguments and routes to either [`add()`] or [`remove()`].
pub fn favorite(mode: &str, command: &str) {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    match mode {
        "add" => add(command),
        "remove" => remove(command),
        _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
    }
}

/// Adds a command to the favorites list.
///
/// # Behavior
/// - Validates command exists
/// - Ensures favorites file exists
/// - Appends command to favorites file
fn add(cmd: &str) {
    let (blue, green, yellow, reset) = (COLORS.blue, COLORS.green, COLORS.yellow, COLORS.reset);
    let path = format!("{}/.local/share/cmdcreate/favorites", VARS.home);

    // Validate command
    is_command_installed(cmd);

    // Ensure file exists
    create_file(&path);

    // Check for duplicates
    if read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .any(|c| c == cmd)
    {
        println!("{yellow}Command {blue}\"{cmd}\"{yellow} is already in favorites.{reset}");
        return;
    }

    // Write contents to file
    write_to_file(&path, &format!("{cmd}\n"));

    // Report success
    println!("{green}Command {blue}\"{cmd}\"{green} added to favorites.{reset}");
}

fn remove(cmd: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    // Remove command from file
    remove_from_file(
        &format!("{}/.local/share/cmdcreate/favorites", VARS.home),
        cmd,
    );

    // Report success
    println!("{green}Command {blue}\"{cmd}\"{green} removed from favorites.{reset}");
}
