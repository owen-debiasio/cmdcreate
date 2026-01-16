use crate::{
    cmds::tools::{get_installed_commands, is_command_installed},
    utils::{
        colors::COLORS,
        fs::{PATHS, delete_file, path_exists, read_file_to_string, remove_from_file},
        io::ask_for_confirmation,
        sys::run_shell_command,
    },
};

pub fn remove(command: &str) {
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    if get_installed_commands().is_empty() {
        return;
    }

    is_command_installed(command);

    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{command}\"{red}?{reset}"
    ));

    delete_file(&format!("{}{command}", PATHS.install_dir));

    if read_file_to_string(&PATHS.favorites).contains(command) && path_exists(&PATHS.favorites) {
        remove_from_file(&PATHS.favorites, command);
    }

    run_shell_command(&format!("sudo rm -f /usr/bin/{command}"));

    println!("\n{green}Removed command {blue}\"{command}\"{reset}");
}
