use crate::{
    commands::tools::get_installed_commands,
    logger::log,
    utils::{colors::COLORS, io::error},
};

pub fn search(cmd: &str) {
    let (yellow, blue, reset) = (COLORS.yellow, COLORS.blue, COLORS.reset);

    let mut count = 0;
    for script in get_installed_commands() {
        let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

        log(
            &format!(
                "cmds/search::search(): Determining if command \"{file_stem}\" matches \"{cmd}\"..."
            ),
            0,
        );

        if file_stem.contains(cmd) {
            if count == 0 {
                println!("--------");
            }

            log(
                &format!("cmds/search::search(): Found match: \"{cmd}\"..."),
                0,
            );

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
        log(
            &format!("cmds/search::search(): Found only 1 match for command \"{cmd}\"..."),
            0,
        );

        println!("Found one match for {blue}\"{cmd}\"{reset}");

        return;
    }

    log(
        &format!("cmds/search::search(): Found multiple matches for command \"{cmd}\"..."),
        0,
    );

    println!("Found {blue}{count}{reset} matches for {blue}\"{cmd}\"{reset}.");
}
