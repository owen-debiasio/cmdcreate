use crate::{
    commands::tools::command_is_installed,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(cmd: &str) {
    command_is_installed(cmd);

    log(
        "commands/display::display(): Printing contents of command...",
        0,
    );

    println!(
        "Contents of command: {}\"{cmd}\"{}\n--------\n{}",
        COLORS.blue,
        COLORS.reset,
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim() // Remove extra whitespace just in case
    );
}
