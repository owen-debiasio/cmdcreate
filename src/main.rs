#![allow(clippy::cargo_common_metadata, reason = "")]
#![allow(clippy::multiple_crate_versions, reason = "")]
#![allow(clippy::too_many_lines, reason = "")]

mod commands;
mod configs;
mod init;
mod logger;
mod utils;

use crate::{
    commands::{
        create::create,
        display::display,
        edit::edit,
        export::export,
        favorite::favorite,
        import::import,
        list::list,
        remove::remove,
        rename::rename,
        repair::repair,
        search::search,
        update::{check, update},
    },
    init::init,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, init_git_fs, read_file_to_string},
        io::error,
        sys::return_args,
    },
};

pub const VERSION: &str = "v1.0.3";

pub fn display_usage() {
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    log("main::display_usage(): Displaying usage information...", 0);

    for line in vec![
        format!("cmdcreate {VERSION}"),
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
        "   Cmdcreate allows you to create custom commands for your Linux terminal".into(),
        "   without needing to enter the same \"complex\" commands over and over.".into(),
    ] {
        println!("{line}");
    }
}

fn main() {
    init();

    log("main::main(): Retrieving args...", 0);

    let mut args = return_args();
    if args.is_empty() {
        display_usage();
        return;
    }

    if args
        .iter()
        .any(|a| matches!(a.as_str(), "-V" | "--verbose"))
    {
        return args.retain(|a| !matches!(a.as_str(), "-V" | "--verbose"));
    }

    if args.is_empty() {
        log("main::main(): Args are empty, displaying usage...", 1);
        display_usage();
        return;
    }

    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let cmd = args.first().unwrap().as_str();
    let arg = |i| return args.get(i).map(String::as_str);

    let (magenta, green, blue, yellow, reset) = (
        COLORS.magenta,
        COLORS.green,
        COLORS.blue,
        COLORS.yellow,
        COLORS.reset,
    );

    if matches!(cmd, "-V" | "--verbose") {
        return;
    }

    if matches!(
        cmd,
        "-l" | "--license" | "-c" | "--changelog" | "-g" | "--get_offline_files"
    ) {
        log("main::main(): Offline files have been requested...", 0);
        init_git_fs();
    }

    log(&format!("main::main(): Processing command \"{cmd}\"..."), 0);

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
        "check" => check(),
        "update" => update(),

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

            log("main::main(): Retrieval of offline files requested...", 0);

            init_git_fs();

            log("main::main(): Retrieved offline files...", 0);

            println!("{green}Files downloaded successfully.{reset}");
        }

        "--remove_offline_files" | "-r" => {
            log("main::main(): Removing offline files...", 0);

            println!("Removing files...");

            delete_file(&PATHS.changelog);
            delete_file(&PATHS.license);

            log("main::main(): Removed offline files", 0);

            println!("{green}Files removed successfully.{reset}");
        }

        "--license" | "-l" => {
            log("main::main(): Displaying license...", 0);
            println!("{}", read_file_to_string(&PATHS.license));
        }

        "--changelog" | "-c" => {
            log("main::main(): Displaying changelog...", 0);
            println!("{}", read_file_to_string(&PATHS.changelog).trim());
        }

        "--debugging" | "-d" => {
            log("main::main(): Displaying debug info...", 0);

            for line in [
                format!("Usage: cmdcreate {magenta}(flags){reset} [run]"),
                format!(
                    "  {magenta}-F{reset}, {magenta}--force_system_shell{reset}          Force system shell"
                ),
                format!(
                    "  {magenta}-f{reset}, {magenta}--force{reset}                       Force commands"
                ),
                format!(
                    "  {magenta}-V{reset}, {magenta}--verbose{reset}                     Print logs"
                ),
            ] {
                println!("{line}");
            }
        }

        _ if cmd.starts_with('-') => {
            error("Invalid argument:", cmd);
        }

        _ => {
            error("Invalid command:", cmd);
        }
    }
}
