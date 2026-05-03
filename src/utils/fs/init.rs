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
    logger::{Severity, log},
    utils::{
        fs::{
            core::{create_file, create_folder, path_exists},
            paths::{MAIN_PATH, PATHS},
        },
        io::error,
        sys::env::running_as_root,
    },
};

pub fn init_filesystem() {
    let favorites_file = &PATHS.favorites;
    let config_file = PATHS.configuration_file;

    log(
        "utils/fs::init::init_filesystem(): Initializing filesystem...",
        Severity::Normal,
    );

    if path_exists(favorites_file) && path_exists(config_file) && path_exists(MAIN_PATH) {
        return;
    }

    if !running_as_root() {
        error(
            "Looks like you're running cmdcreate for the first time, \
            please run cmdcreate as root to set up the filesystem.",
            "",
        )
    }

    create_folder(MAIN_PATH);
    create_file(favorites_file);
    create_file(config_file);

    if !(path_exists(favorites_file) && path_exists(config_file) && path_exists(MAIN_PATH)) {
        error(
            "Failed to initialize filesystem!",
            "Please make sure your are running as root!",
        )
    }

    log(
        "utils/fs::init::init_filesystem(): Filesystem initialized",
        Severity::Normal,
    );
}
