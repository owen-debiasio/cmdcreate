use crate::{
    cmds::tools::is_command_installed,
    utils::{fs::PATHS, msgs::error, sys::run_shell_command},
};
use std::process::Command;

pub const SUPPORTED_EDITORS: [&str; 16] = [
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
    "zed-beta",
    "mousepad",
];

pub fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn edit(cmd: &str) {
    is_command_installed(cmd);

    let Some(editor) = SUPPORTED_EDITORS
        .iter()
        .find(|&&ed| is_editor_installed(ed))
    else {
        error("No known editor is installed on your device.", "");
        return;
    };

    run_shell_command(&format!("sudo {editor} {}{cmd}", PATHS.install_dir));
}
