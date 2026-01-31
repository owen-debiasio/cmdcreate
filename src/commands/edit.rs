use crate::{
    commands::tools::is_command_installed,
    logger::log,
    utils::{
        fs::PATHS,
        io::error,
        sys::{VARS, run_shell_command},
    },
};
use std::process::Command;

pub fn edit(cmd: &str) {
    log(
        &format!("cmds/edit::edit(): Checking if command \"{cmd}\" is installed..."),
        0,
    );

    is_command_installed(cmd);

    log("cmds/edit::edit(): Checking editors...", 0);

    let editors: &[&str] = &[
        &VARS.editor, // Used when user runs something like "EDITOR=vi cmdcreate edit abc"
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
    ];

    let editor = editors.iter().find(|&&ed| {
        Command::new("which")
            .arg(ed)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    });

    let editor = if let Some(ed) = editor {
        *ed
    } else {
        error("No known editor is installed on your device.", "");
        return;
    };

    log(
        &format!("cmds/edit::edit(): Launching editor \"{editor}\"..."),
        0,
    );

    run_shell_command(&format!("sudo {editor} {}{cmd}", PATHS.install_dir));
}
