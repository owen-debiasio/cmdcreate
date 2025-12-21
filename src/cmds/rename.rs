use crate::{
    cmds::tools::{get_installed_commands, is_command_installed},
    utils::{colors::COLORS, fs::PATHS, sys::run_shell_command},
};

pub fn rename(old: &str, new: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);
    let install_dir = &PATHS.install_dir;

    if get_installed_commands().is_empty() {
        return;
    }

    is_command_installed(old);

    run_shell_command(&format!(
        "
        mv {install_dir}{old} {install_dir}{new}; \
        sudo mv /usr/bin/{old} /usr/bin/{new}; \
        sudo ln -sf {install_dir}{new} /usr/bin/{new}; \
        "
    ));

    println!("{green}Successfully renamed command {blue}\"{old}\" to {blue}\"{new}\"{reset}");
}
