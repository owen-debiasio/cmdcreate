mod cmds;
mod configs;
mod init;
mod utils;

use crate::{
    cmds::{
        backup::{export::export, import::import},
        create::create,
        display::display,
        edit::edit,
        favorite::favorite,
        list::list,
        remove::remove,
        rename::rename,
        repair::repair,
        search::search,
        upgrader::{check_for_updates, upgrade},
    },
    init::init,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, init_git_fs, read_file_to_string},
        msgs::{display_usage, error},
        sys::return_args,
    },
};

pub const VERSION: &str = "v0.9.9";

fn main() {
    init();

    let args = return_args();
    if args.is_empty() {
        display_usage();
        return;
    }

    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let cmd = args[0].as_str();
    let arg = |i| args.get(i).map(String::as_str);

    let (magenta, green, blue, yellow, reset) = (
        COLORS.magenta,
        COLORS.green,
        COLORS.blue,
        COLORS.yellow,
        COLORS.reset,
    );

    if matches!(cmd, "-l" | "--license" | "-c" | "--changelog") {
        init_git_fs();
    }

    match cmd {
        "create" => match (arg(1), arg(2)) {
            (Some(c), Some(v)) => create(c, v, true),
            _ => println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}"),
        },

        "remove" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}"),
            remove,
        ),

        "edit" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}edit {yellow}<command>{reset}"),
            edit,
        ),

        "search" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}search {yellow}<command>{reset}"),
            search,
        ),

        "display" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}"),
            display,
        ),

        "rename" => match (arg(1), arg(2)) {
            (Some(a), Some(b)) => rename(a, b),
            _ => println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}"),
        },

        "favorite" => match (arg(1), arg(2)) {
            (Some(op @ ("add" | "remove")), Some(c)) => favorite(op, c),
            _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
        },

        "repair" => repair(),
        "list" => list(),
        "check" => check_for_updates(),
        "update" => upgrade(),

        "import" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}"),
            import,
        ),

        "export" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}export {yellow}<output dir>{reset}"),
            export,
        ),

        "--version" | "-v" => {
            println!("cmdcreate {VERSION}\n(C) 2026 Owen Debiasio; Licensed under GPL-2.0-only");
        }

        "--get_offline_files" | "-g" => {
            println!("Downloading offline files...");
            init_git_fs();
            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            println!("Removing files...");
            delete_file(&PATHS.changelog);
            delete_file(&PATHS.license);
            println!("{green}Files removed successfully.{reset}");
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
