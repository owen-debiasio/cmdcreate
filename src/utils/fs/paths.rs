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
    core::meta::license::get_normal_license_paths,
    utils::sys::env::{ENVIRONMENT_VARIABLES, running_as_root},
};
use std::{path::Path, sync::LazyLock};

pub static MAIN_PATH: LazyLock<&str> = LazyLock::new(|| {
    if running_as_root() {
        "/usr/share/cmdcreate"
    } else {
        "~/.local/share/cmdcreate"
    }
});

pub struct Paths {
    pub configuration_file: &'static str,
    pub favorites: String,
    pub command_installation_directory: &'static str,
    pub license: String,
    pub log_directory: &'static str,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    configuration_file: if running_as_root() {
        "/etc/cmdcreate.toml"
    } else {
        "~/.config/cmdcreate/cmdcreate.toml"
    },
    favorites: format!("{}/favorites", &MAIN_PATH.to_string()),
    command_installation_directory: if running_as_root() {
        "/usr/local/bin/"
    } else {
        "~/.local/bin/cmdcreate/"
    },
    license: if running_as_root() {
        get_normal_license_paths().to_string()
    } else {
        format!("{}/LICENSE", &MAIN_PATH.to_string())
    },
    log_directory: "/tmp/cmdcreate-logs/",
});

pub fn expand_home_dir(apparent_path: &str) -> String {
    apparent_path.replace('~', &ENVIRONMENT_VARIABLES.home)
}

pub fn path_exists(apparent_path: &str) -> bool {
    let path = expand_home_dir(apparent_path);
    Path::new(&path).exists()
}

#[test]
fn path_exists_reports_correctly() {
    use crate::utils::fs::core::creation::{create_file, delete_file};

    let file_name = "cmdcreate_test_file";
    create_file(file_name);

    assert!(path_exists(file_name));

    delete_file(file_name);
}
