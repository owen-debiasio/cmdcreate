use crate::{
    cmds::tools::{get_installed_commands, is_command_installed},
    logger::log,
    utils::{colors::COLORS, fs::PATHS, sys::run_shell_command},
};

pub fn rename(old: &str, new: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);
    let install_dir = &PATHS.install_dir;

    if get_installed_commands().is_empty() {
        log("cmds/rename::rename(): Command is empty, exiting...", 0);
        return;
    }

    log(
        &format!("cmds/rename::rename(): Determining if old command \"{old}\" is installed..."),
        0,
    );

    is_command_installed(old);

    log(
        &format!("cmds/rename::rename(): Determining if new command \"{new}\" is installed..."),
        0,
    );

    is_command_installed(new);

    log(
        &format!("cmds/rename::rename(): Renaming command \"{old}\" to \"{new}\"..."),
        0,
    );

    run_shell_command(&format!(
        "
        mv {install_dir}{old} {install_dir}{new}; \
        sudo mv /usr/local/bin/{old} /usr/local/bin/{new}; \
        sudo ln -sf {install_dir}{new} /usr/local/bin/{new}; \
        "
    ));

    println!("{green}Successfully renamed command {blue}\"{old}\" to {blue}\"{new}\"{reset}");
}
