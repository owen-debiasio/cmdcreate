use crate::{
    commands::{favorite::favorite, tools::command_is_installed},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, path_exists, read_file_to_string},
        io::ask_for_confirmation,
        sys::run_shell_command,
    },
};

pub fn remove(command: &str, forced: bool) {
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    if command.is_empty() {
        log("commands/remove::remove(): Command is empty, exiting...", 0);
        return;
    }

    log(
        &format!("commands/remove::remove(): Determining if command \"{command}\" is installed..."),
        0,
    );

    command_is_installed(command);

    log(
        &format!(
            "commands/remove::remove(): Asking for confirmation to delete command \"{command}\"..."
        ),
        0,
    );

    if !forced {
        ask_for_confirmation(&format!(
            "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
        ));
    }

    log(
        &format!("commands/remove::remove(): Deleting command \"{command}\"..."),
        0,
    );

    delete_file(&format!("{}{command}", PATHS.install_dir));

    log(
        &format!(
            "commands/remove::remove(): Determining if command \"{command}\" is in favorites..."
        ),
        0,
    );

    if read_file_to_string(&PATHS.favorites).contains(command) && path_exists(&PATHS.favorites) {
        log(
            &format!(
                "commands/remove::remove(): Command \"{command}\" is in favorites, removing..."
            ),
            0,
        );

        favorite("remove", command);
    }

    log(
        &format!(
            "commands/remove::remove(): Command \"{command}\" is not in favorites, skipping..."
        ),
        1,
    );

    log(
        &format!("commands/remove::remove(): Removing link of command \"{command}\"..."),
        0,
    );

    run_shell_command(&format!("sudo rm -f /usr/local/bin/{command}"));

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
