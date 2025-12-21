mod cmds;
mod utils;

use crate::{
    cmds::{
        backup::{export, import},
        create,
        edit::edit,
        favorite, list, remove, rename, repair, search,
        upgrader::{check_for_updates, upgrade},
    },
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, get_files, read_file_to_string},
        msgs::{display_usage, error},
        sys::{ctrlc_enabled, return_args},
    },
};

pub const VERSION: &str = "v0.8.7";

fn main() {
    let args = return_args();
    if args.is_empty() {
        display_usage();
        return;
    }

    ctrlc_enabled(false);
    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let Some(cmd) = args.first() else {
        error("No command provided", "");
        return;
    };

    let (magenta, green, blue, yellow, reset) = (
        COLORS.magenta,
        COLORS.green,
        COLORS.blue,
        COLORS.yellow,
        COLORS.reset,
    );

    if matches!(cmd.as_str(), "-l" | "--l" | "-c" | "--c") {
        get_files();
    }

    let arg = |i| args.get(i).map(String::as_str);

    match cmd.as_str() {
        "create" => match (arg(1), arg(2)) {
            (Some(c), Some(v)) => create::create(c, v, true),
            _ => println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}"),
        },

        "remove" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}"),
            remove::remove,
        ),

        "edit" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}edit {yellow}<command>{reset}"),
            edit,
        ),

        "search" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}"),
            search::search,
        ),

        "display" => {
            if let Some(cmd) = arg(1) {
                println!(
                    "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
                    read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim()
                );
            } else {
                println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}");
            }
        }

        "rename" => match (arg(1), arg(2)) {
            (Some(a), Some(b)) => rename::rename(a, b),
            _ => println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}"),
        },

        "favorite" => match (arg(1), arg(2)) {
            (Some(op @ ("add" | "remove")), Some(cmd)) => favorite::favorite(op, cmd),
            _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
        },

        "repair" => repair::repair(),
        "list" => list::list(),
        "check" => check_for_updates(),
        "update" => upgrade(),

        "import" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}"),
            import::import,
        ),

        "export" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}export {yellow}<output dir>{reset}"),
            export::export,
        ),

        "--version" | "-v" => println!("cmdcreate {VERSION}\n(C) 2025 Owen Debiasio; MIT License"),

        "--get_offline_files" | "-g" => {
            println!("Downloading offline files...");
            get_files();
            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            delete_file(&PATHS.changelog);
            delete_file(&PATHS.license);
            println!("Files removed successfully.");
        }

        "--license" | "-l" => println!("{}", read_file_to_string(&PATHS.license)),
        "--changelog" | "-c" => println!("{}", read_file_to_string(&PATHS.changelog).trim()),

        "--debugging" | "-d" => {
            for line in [
                format!("Usage: cmdcreate {magenta}(flags){reset} [run]"),
                format!("  {magenta}-F{reset}, --force_system_shell"),
                format!("  {magenta}-f{reset}, --force"),
            ] {
                println!("{line}");
            }
        }

        _ if cmd.starts_with('-') => error("Invalid argument:", cmd),
        _ => error("Invalid command:", cmd),
    }
}
