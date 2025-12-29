use crate::{
    cmds::tools::get_installed_commands,
    utils::{colors::COLORS, msgs::error},
};

pub fn search(cmd: &str) {
    let (yellow, blue, reset) = (COLORS.yellow, COLORS.blue, COLORS.reset);

    let mut count = 0;
    for script in get_installed_commands() {
        let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

        if file_stem.contains(cmd) {
            if count == 0 {
                println!("--------");
            }

            println!("{file_stem}");
            count += 1;
        }
    }

    if count == 0 {
        error(
            "No installed commands contain:",
            &format!("{yellow}\"{cmd}\"{reset}"),
        );
    }

    println!("--------");

    if count == 1 {
        println!("Found one match for {blue}\"{cmd}\"{reset}");
        return;
    }

    println!("Found {blue}{count}{reset} matches for {blue}\"{cmd}\"{reset}.");
}
