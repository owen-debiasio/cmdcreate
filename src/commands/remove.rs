use crate::{
    commands::{favorite::favorite, tools::command_is_installed},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, read_file_to_string},
        io::ask_for_confirmation,
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

    command_is_installed(command);

    if !forced {
        ask_for_confirmation(&format!(
            "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
        ));
    }

    if read_file_to_string(&PATHS.favorites).contains(command) {
        favorite("remove", command);
    }

    log(
        &format!("commands/remove::remove(): Removing command \"{command}\"..."),
        0,
    );

    delete_file(&format!("{}{command}", PATHS.install_dir));

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
