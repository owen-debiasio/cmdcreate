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
    commands::{create::create, favorite::favorite as add_favorite},
    logger::log,
    utils::{colors::COLORS, fs::read_file_to_string, io::error},
};

pub fn import(file: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/import::import(): Importing commands from file: \"{file}\"..."),
        0,
    );

    let contents = read_file_to_string(file).expect("Failed to get file contents");

    if contents.trim().is_empty() {
        error("Import file is empty or unreadable.", "");
    }

    for line in contents.replace("[|", "|").lines() {
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        if line.trim().is_empty() && !parts.is_empty() {
            continue;
        }

        let name = parts.first().unwrap();
        let mut data = String::new();
        let mut favorite = false;

        for part in parts.iter().skip(1) {
            if *part == "favorite" {
                favorite = true;
            } else {
                if data.is_empty() {
                    data.push('\n');
                }

                data.push_str(part);
            }
        }

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        create(name, &data, false);

        if favorite {
            add_favorite("add", name);
        }
    }

    println!("\n{green}Successfully imported commands.{reset}");
}
