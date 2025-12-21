use std::{
    env::{args, var},
    process::{Command, Stdio, exit},
    sync::LazyLock,
};

use crate::utils::msgs::error;
use ctrlc::set_handler;
pub struct Vars {
    pub shell: String,
    pub home: String,
}

pub static VARS: std::sync::LazyLock<Vars> = LazyLock::new(|| Vars {
    shell: var("SHELL").unwrap_or_else(|_| "unknown".to_string()),
    home: var("HOME").unwrap_or_else(|_| "unknown".to_string()),
});

pub fn return_args() -> Vec<String> {
    args().skip(1).collect()
}

pub fn args_contains(arg: &str) -> bool {
    return_args().contains(&arg.to_string())
}

pub fn run_shell_command(cmd: &str) {
    let shell: String = if args_contains("--force_system_shell") | args_contains("-F") {
        VARS.shell.clone()
    } else {
        "bash".to_string()
    };

    if cmd.trim().is_empty() {
        return;
    }

    match Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {}
        Err(e) => {
            error("Failed to run shell command:", &e.to_string());
        }
    }
}

#[allow(clippy::doc_overindented_list_items)]
pub fn ctrlc_enabled(enabled: bool) {
    if enabled {
        set_handler(|| exit(1)).expect("Failed to set Ctrl-C handler");
        return;
    }

    set_handler(|| {}).expect("Failed to set Ctrl-C handler");
}
