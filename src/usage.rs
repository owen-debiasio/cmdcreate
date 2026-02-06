use crate::{logger::log, utils::colors::COLORS, version::VERSION};

pub fn cmdcreate_usage() {
    let (blue, cyan, yellow, magenta, reset) = (
        COLORS.blue,
        COLORS.cyan,
        COLORS.yellow,
        COLORS.magenta,
        COLORS.reset,
    );

    log(
        "usage::cmdcreate_usage(): Displaying usage information...",
        0,
    );

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

pub fn debug_usage() {
    let (magenta, reset) = (COLORS.magenta, COLORS.reset);

    log("usage::debug_usage(): Displaying debug info...", 0);

    for line in [
        format!("Usage: cmdcreate {magenta}(flags){reset} [run]"),
        format!(
            "  {magenta}-f{reset}, {magenta}--force{reset}                       Force commands"
        ),
        format!("  {magenta}-V{reset}, {magenta}--verbose{reset}                     Print logs"),
    ] {
        println!("{line}");
    }
}
