use crate::{
    cmds::tools::is_command_installed,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn display(cmd: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);
    
    is_command_installed(cmd);
    
    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).trim()
    );
}
