// Copyright (C) 2026 Owen Debiasio
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, overwrite_file},
        io::error,
        sys::run_shell_command,
    },
};

pub fn create(command: &str, contents: &str, verbose: bool) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "commands/create::create(): Creating command \"{command}\": With contents \"{contents}\"{}",
            if verbose {
                ", and is verbose..."
            } else {
                "..."
            }
        ),
        0,
    );

    let script = &format!("{}{command}", PATHS.install_dir);

    log(
        &format!("commands/create::create(): Script path: \"{script}\""),
        0,
    );

    if contents.is_empty() {
        error("The contents of your command can not be empty.", "");
    }

    log(
        &format!("commands/create::create(): Writing contents to script: \"{script}\""),
        0,
    );

    overwrite_file(script, contents).expect("Failed to write to file");

    log("commands/create::create(): Activating command...", 0);

    run_shell_command(&format!("chmod +x {script}"));

    if verbose {
        println!("\n{green}Success! Created command: {blue}\"{command}\"{reset}");
    }
}
