use crate::{
    commands::{favorite::favorite, tools::command_is_installed},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, read_file_to_string},
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

    command_is_installed(command);

    if !forced {
        ask_for_confirmation(&format!(
            "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
        ));
    }

    log(
        &format!("commands/remove::remove(): Deleting command file \"{command}\"..."),
        0,
    );

    delete_file(&format!("{}{command}", PATHS.install_dir));

    if read_file_to_string(&PATHS.favorites).contains(command) {
        favorite("remove", command);
    }

    log(
        &format!("commands/remove::remove(): Removing link of command \"{command}\"..."),
        0,
    );

    run_shell_command(&format!("sudo rm -f /usr/local/bin/{command}"));

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
