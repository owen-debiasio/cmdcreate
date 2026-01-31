use crate::utils::sys::args_forced;
use crate::{
    VERSION,
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
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, init_git_fs, read_file_to_string},
        io::error,
    },
};

pub fn parse(cmd: &str, args: &[String]) {
    let (magenta, green, blue, yellow, reset) = (
        COLORS.magenta,
        COLORS.green,
        COLORS.blue,
        COLORS.yellow,
        COLORS.reset,
    );

    let arg = |i| args.get(i).map(String::as_str);

    match cmd {
        "create" => match (arg(1), arg(2)) {
            (Some(command), Some(contents)) => create(command, contents, true),
            _ => println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}"),
        },

        "remove" => arg(1).map_or_else(
            || println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}"),
            |cmd| remove(cmd, args_forced()),
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
            (Some(command), Some(new_name)) => rename(command, new_name),
            _ => println!("Usage:\ncmdcreate {blue}rename {yellow}<command> <new name>{reset}"),
        },

        "favorite" => match (arg(1), arg(2)) {
            (Some(mode @ ("add" | "remove")), Some(command)) => favorite(mode, command),
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
            println!("cmdcreate {VERSION}\n(C) 2026 Owen Debiasio; Licensed under MIT");
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
