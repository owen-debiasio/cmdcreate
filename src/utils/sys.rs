/// System Utilities for cmdcreate
///
/// This module provides system-level helper functions used throughout cmdcreate.
/// It includes argument handling, shell command execution, and environment variable
/// access. These utilities abstract OS-level interactions for consistency and safety.
///
/// # Features
/// - Access environment variables like `SHELL` and `HOME`
/// - Parse command-line arguments
/// - Check for specific flags/arguments
/// - Run shell commands safely with optional system shell override
use std::{
    env,
    process::{Command, Stdio},
};

use once_cell::sync::Lazy;

use crate::utils::msgs::error;

/// Holds key environment variables for cmdcreate
pub struct Vars {
    /// Current shell of the user
    pub shell: String,
    /// User's home directory
    pub home: String,
}

/// Global static variable storing shell and home paths
pub static VARS: Lazy<Vars> = Lazy::new(|| Vars {
    shell: env::var("SHELL").unwrap_or_else(|_| "unknown".to_string()), // fallback if SHELL not set
    home: env::var("HOME").unwrap_or_else(|_| "unknown".to_string()),   // fallback if HOME not set
});

/// Returns the command-line arguments passed to the program
///
/// Skips the first argument (program name) automatically
///
/// # Returns
/// * `Vec<String>` - List of arguments
pub fn return_args() -> Vec<String> {
    env::args()
        .skip(1) // skip program name
        .collect()
}

/// Checks if a specific argument or flag is present
///
/// # Arguments
/// * `s` - The argument or flag to check (e.g., "--force")
///
/// # Returns
/// * `bool` - true if the argument exists
pub fn args_contains(s: &str) -> bool {
    return_args().contains(&s.to_string())
}

/// Executes a shell command in a child process
///
/// # Arguments
/// * `cmd` - The command string to run
///
/// Behavior:
/// - Uses the user's default shell unless `--force_system_shell` or `-F` is passed
/// - Inherits stdin, stdout, and stderr from the parent process
/// - Safely handles empty commands and errors
pub fn run_shell_command(cmd: &str) {
    // Determine which shell to use
    let shell: String = if args_contains("--force_system_shell") | args_contains("-F") {
        VARS.shell.clone() // use user's default shell
    } else {
        "bash".to_string() // fallback to bash
    };

    // Skip empty commands
    if cmd.trim().is_empty() {
        return;
    }

    // Execute the command
    match Command::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {} // Command executed successfully
        Err(e) => {
            // Report error
            error("Failed to run shell command:", &e.to_string());
        }
    }
}
