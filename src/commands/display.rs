use crate::{
    commands::tools::is_command_installed,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(cmd: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log(
        &format!("cmds/display::display(): Checking if command \"{cmd}\" is installed..."),
        0,
    );

    is_command_installed(cmd);

    log(
        "cmds/display::display(): Printing contents of command...",
        0,
    );

    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim()
    );
}
