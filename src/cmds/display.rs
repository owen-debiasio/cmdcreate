use crate::{
    cmds::tools::is_command_installed,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(c: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);
    is_command_installed(c);
    println!(
        "Contents of command: {blue}\"{c}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}{c}", PATHS.install_dir)).trim()
    );
}
