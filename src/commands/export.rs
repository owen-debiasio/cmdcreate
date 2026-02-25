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
    commands::tools::get_installed_commands,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, create_file, read_file_to_string, write_to_file},
    },
};

pub fn export(path: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let export_file = &format!("{path}/export.cmdcreate");

    log(
        &format!("commands/export::export(): Exporting commands to: \"{export_file}\"..."),
        0,
    );

    log("commands/export::export(): Creating export file...", 0);

    create_file(export_file).expect("Failed to create file");

    for script in get_installed_commands() {
        if let Some(stem) = script.file_stem() {
            let cmd = stem.to_string_lossy();

            log(
                &format!("commands/export::export(): Exporting command: \"{cmd}\"..."),
                0,
            );

            let cmd_contents = read_file_to_string(&format!("{}{cmd}", PATHS.install_dir))
                .expect("Failed to retrieve command contents")
                .replace('|', "[|");

            let data = if read_file_to_string(&PATHS.favorites)
                .expect("Failed to retrieve favorites")
                .contains(cmd.as_ref())
            {
                format!("{cmd} | {cmd_contents} | favorite\n")
            } else {
                format!("{cmd} | {cmd_contents}\n")
            };

            log(
                &format!("commands/export::export(): Writing data to file: \"{data}\"..."),
                0,
            );

            write_to_file(export_file, &data, true).expect("Failed to write to file");
        }
    }

    println!("{green}Successfully exported commands to:{blue} \"{export_file}\"{green}.{reset}",);
}
