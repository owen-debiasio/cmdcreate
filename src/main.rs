mod commands;
mod configs;
mod init;
mod logger;
mod parse;
mod utils;

use crate::{
    init::init,
    logger::log,
    parse::parse,
    utils::{colors::COLORS, fs::init_git_fs, sys::return_args},
};

pub const VERSION: &str = "v1.0.7";

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
        args.retain(|a| !matches!(a.as_str(), "-V" | "--verbose"));
    }

    if args.is_empty() {
        log("main::main(): Args are empty, displaying usage...", 1);
        display_usage();
        return;
    }

    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let mut i = 0;

    while let Some(cmd) = args.get(i).map(String::as_str) {
        if !cmd.starts_with('-') {
            break;
        }

        if matches!(cmd, "-V" | "--verbose") {
            i += 1;
            continue;
        }

        if matches!(
            cmd,
            "-l" | "--license" | "-c" | "--changelog" | "-g" | "--get_offline_files"
        ) {
            log("main::main(): Offline files have been requested...", 0);
            init_git_fs();
        }

        log(&format!("main::main(): Processing command \"{cmd}\"..."), 0);
        parse(cmd, args);

        i += 1;
    }

    if let Some(cmd) = args.get(i).map(String::as_str) {
        log(&format!("main::main(): Processing command \"{cmd}\"..."), 0);
        parse(cmd, &args[i..]);
    }
}
