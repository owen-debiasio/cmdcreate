/// Module for repairing broken cmdcreate command installations
///
/// This checks for commands that should exist in `/usr/bin` but don’t,
/// then rebuilds their symlinks/files automatically using `create()`.
use crate::utils::fs::path_exists;
use crate::{
    cmds::{create::create, tools::retrieve_commands}, // Retrieve installed scripts + rebuild them
    utils::colors::COLORS,                            // Terminal color formatting
};

/// Scans installed commands and restores any missing system links
///
/// The function:
/// 1. Retrieves all installed scripts
/// 2. Checks if each script has a corresponding `/usr/bin/<name>`
/// 3. If missing, it regenerates it using `create()`
///
/// Outputs a summary showing how many commands were repaired.
pub fn repair() {
    // Color codes for pretty terminal output
    let (green, yellow, reset) = (COLORS.green, COLORS.yellow, COLORS.reset);

    // Tracks how many commands got fixed
    let mut count = 0;

    // Iterate through all installed scripts
    for script in retrieve_commands("installed") {
        // Extract just the file name (command name)
        let name = script.file_stem().unwrap_or_default().to_string_lossy();

        // If the command isn't linked into /usr/bin, repair it
        if !path_exists(&format!("/usr/bin/{name}")) {
            println!("Repairing command: {name}");

            // Recreate the command symlink/script
            create(&name, "", false);

            count += 1;
        }
    }

    // Final status message depending on if repairs were needed
    if count > 0 {
        println!("{green}Broken commands have been repaired.{reset}")
    } else {
        println!("{yellow}No commands needed repairs.{reset}");
    }
}
