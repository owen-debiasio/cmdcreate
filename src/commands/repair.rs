use crate::{
    commands::{create::create, tools::get_installed_commands},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists, read_file_to_string},
    },
};

pub fn repair() {
    let (green, yellow, blue, reset) = (COLORS.green, COLORS.yellow, COLORS.blue, COLORS.reset);

    log(
        "commands/repair::repair(): Initializing command repair process...",
        0,
    );

    let mut count = 0_i32;
    for script in get_installed_commands() {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();

        log(
            &format!("commands/repair::repair(): Checking if command \"{name}\" needs repair..."),
            0,
        );

        if !path_exists(&format!("/usr/local/bin/{name}")) {
            log(
                &format!("commands/repair::repair(): Repairing command \"{name}\"..."),
                0,
            );

            println!("{green}Repairing command: {blue}\"{name}\"{reset}");

            create(
                &name,
                &read_file_to_string(&format!("{}{name}", PATHS.install_dir)),
                false,
            );

            count += 1_i32;
        }
    }

    log("commands/repair::repair(): Determining results...", 0);

    if count > 0_i32 {
        println!("{green}Broken commands have been repaired.{reset}");

        log(
            "commands/repair::repair(): Broken commands have been repaired",
            0,
        );

        return;
    }

    log(
        "commands/repair::repair(): No commands needed repairs...",
        0,
    );

    println!("{yellow}No commands needed repairs.{reset}");
}
