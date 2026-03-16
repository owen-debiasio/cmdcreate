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

use crate::utils::{
    fs::{PATHS, path_exists, read_file_to_string},
    io::error,
};

pub const YEAR: &str = "2026";

pub const AUTHOR: &str = "Owen Debiasio";
pub const AUTHOR_USERNAME: &str = "owen-debiasio";
pub const AUTHOR_EMAIL: &str = "owen.debiasio@gmail.com";

pub const PROJECT_REPO: &str = "cmdcreate";

pub fn get_project_copyright_info() -> String {
    // So fucking annoyed that this can't be a static
    format!("Copyright {YEAR} {AUTHOR} <{AUTHOR_EMAIL}>")
}

pub fn display_full_license() {
    let path_to_license_file = &PATHS.license;

    if path_exists(path_to_license_file) {
        let license_file_contents = read_file_to_string(path_to_license_file);

        println!("{license_file_contents}");
    } else {
        error(
            "License has not been installed. Find it here:",
            "https://github.com/owen-debiasio/cmdcreate/blob/main/LICENSE",
        )
    }
}
