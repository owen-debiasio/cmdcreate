use crate::{
    commands::tools::is_command_installed,
    logger::log,
    utils::{fs::PATHS, io::error, sys::run_shell_command},
};
use std::process::Command;

pub fn edit(cmd: &str) {
    log(
        &format!("cmds/edit::edit(): Checking if command \"{cmd}\" is installed..."),
        0,
    );

    is_command_installed(cmd);

    log("cmds/edit::edit(): Checking editors...", 0);

    let Some(editor) = [
        "nvim",
        "vi",
        "vim",
        "nano",
        "micro",
        "code",
        "code-insiders",
        "gedit",
        "kate",
        "kwrite",
        "emacs",
        "vscodium",
        "vscodium-insiders",
        "zed",
        "zed-preview",
        "mousepad",
    ]
    .iter()
    .find(|&&ed| {
        Command::new("which")
            .arg(ed)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }) else {
        error("No known editor is installed on your device.", "");

        return;
    };

    log(
        &format!("cmds/edit::edit(): Launching editor \"{editor}\"..."),
        0,
    );

    run_shell_command(&format!("sudo {editor} {}{cmd}", PATHS.install_dir));
}
