use crate::{
    commands::{
        remove::remove,
        tools::{get_installed_commands, is_command_installed},
    },
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, path_exists},
        io::{error, input},
        sys::{args_forced, run_shell_command},
    },
};

pub fn rename(old: &str, new: &str) {
    let (blue, red, yellow, green, reset) = (
        COLORS.blue,
        COLORS.red,
        COLORS.yellow,
        COLORS.green,
        COLORS.reset,
    );
    let (install_dir, installed_commands) = (&PATHS.install_dir, get_installed_commands());

    if installed_commands.is_empty() {
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

    if path_exists(&format!("{}/{new}", PATHS.install_dir)) {
        log(
            &format!(
                "cmds/rename::rename(): Command \"{new}\" is installed... Requesting removal..."
            ),
            0,
        );

        println!(
            "{red}The new name ({yellow}{new}{red}) is already installed! Do you want to delete it?\n({green}Y{red} or {yellow}N{red})",
        );

        if input("").to_lowercase() == "y" || args_forced() {
            log("cmds/rename::rename(): Accepting... Continuing...", 0);

            remove(new, true);
        } else {
            error("You need to remove the old command before proceeding!", "");
        }
    }

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
