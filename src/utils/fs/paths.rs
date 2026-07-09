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
    core::meta::license::get_license_location,
    utils::sys::env::{ENVIRONMENT_VARIABLES, running_as_root},
};
use std::{env::current_exe, path::Path, sync::LazyLock};

pub static MAIN_PATH: LazyLock<&str> = LazyLock::new(|| {
    if running_as_root() {
        "/usr/share/cmdcreate"
    } else {
        "~/.local/share/cmdcreate"
    }
});

pub static CMDCREATE_BINARY_PATH: LazyLock<String> = LazyLock::new(|| {
    current_exe()
        .expect("Failed to get the binary path!")
        .to_string_lossy()
        .into_owned()
});

pub fn get_program_binary_path() -> String {
    let user_home = &ENVIRONMENT_VARIABLES.home;

    let paths_to_check = [
        "/usr/bin/cmdcreate".to_string(),
        "/usr/bin/cmdcreate-dev".to_string(),
        format!("{user_home}/.local/bin/cmdcreate"),
        format!("{user_home}/.local/bin/cmdcreate-dev"),
    ];

    paths_to_check
        .into_iter()
        .find(|path| path_exists(path))
        .unwrap_or_else(|| format!("{user_home}/.local/bin/cmdcreate-dev"))
}

pub struct Paths {
    pub configuration_file: &'static str,
    pub favorites: String,
    pub command_installation_directory: &'static str,
    pub license: &'static str,
    pub log_directory: &'static str,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    configuration_file: if running_as_root() {
        "/etc/cmdcreate.toml"
    } else {
        "~/.config/cmdcreate/cmdcreate.toml"
    },
    favorites: format!("{}/favorites", MAIN_PATH.to_string().as_str()),
    command_installation_directory: if running_as_root() {
        "/usr/local/bin/"
    } else {
        "~/.local/bin/cmdcreate/"
    },
    license: get_license_location(),
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
