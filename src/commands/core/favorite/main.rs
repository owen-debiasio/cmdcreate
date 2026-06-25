// SPDX-License-Identifier: GPL-3.0-or-later
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
    commands::core::favorite::{add_favorite::add, remove_favorite::remove},
    utils::{
        fs::{core::read_write::read_file_to_string, paths::PATHS},
        io::error,
    },
};

pub fn command_is_in_favorites(command: &str) -> bool {
    let favorites_path = &PATHS.favorites;
    let favorites_file_contents = read_file_to_string(favorites_path);

    favorites_file_contents.contains(command)
}

pub fn favorite(action: &str, command: &str) {
    match action {
        "add" => add(command),
        "remove" => remove(command),

        _ => error("Invalid option:", Some(action)),
    }
}
