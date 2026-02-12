use crate::{
    commands::tools::command_is_installed,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(cmd: &str) {
    command_is_installed(cmd);

    println!(
        "Contents of command: {}\"{cmd}\"{}\n--------\n{}",
        COLORS.blue,
        COLORS.reset,
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim() // Remove extra whitespace just in case
    );
}
