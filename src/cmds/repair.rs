use crate::{
    cmds::{create::create, tools::get_installed_commands},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{path_exists, read_file_to_string, PATHS},
    },
};

pub fn repair() {
    let (green, yellow, blue, reset) = (COLORS.green, COLORS.yellow, COLORS.blue, COLORS.reset);

    log(
        "cmds/repair::repair(): Initializing command repair process...",
        0,
    );

    let mut count = 0;
    for script in get_installed_commands() {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();

        log(
            &format!("cmds/repair::repair(): Checking if command \"{name}\" needs repair..."),
            0,
        );

        if !path_exists(&format!("/usr/bin/local/{name}")) {
            log(
                &format!("cmds/repair::repair(): Repairing command \"{name}\"..."),
                0,
            );

            println!("{green}Repairing command: {blue}\"{name}\"{reset}");

            create(
                &name,
                &read_file_to_string(&format!("{}{name}", PATHS.install_dir)),
                false,
            );

            count += 1;
        }
    }

    log("cmds/repair::repair(): Determining results...", 0);

    if count > 0 {
        println!("{green}Broken commands have been repaired.{reset}");

        log(
            "cmds/repair::repair(): Broken commands have been repaired",
            0,
        );

        return;
    }

    log("cmds/repair::repair(): No commands needed repairs...", 0);

    println!("{yellow}No commands needed repairs.{reset}");
}
