use crate::{
    commands::tools::command_is_installed,
    logger::log,
    utils::{
        fs::PATHS,
        io::error,
        sys::{VARS, run_shell_command},
    },
};
use std::process::Command;

pub fn edit(cmd: &str) {
    command_is_installed(cmd);

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

    let editor = editor.map_or_else(
        || {
            error("No known editor is installed on your device.", "");
        },
        |ed| *ed,
    );

    log(
        &format!("commands/edit::edit(): Using editor \"{editor}\"..."),
        0,
    );

    run_shell_command(&format!("sudo {editor} {}{cmd}", PATHS.install_dir));
}
