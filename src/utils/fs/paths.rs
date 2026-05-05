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

use crate::utils::sys::distro::{DistroBase, get_distro_base};
use std::sync::LazyLock;

pub static MAIN_PATH: &str = "/usr/share/cmdcreate";

pub struct Paths {
    pub configuration_file: &'static str,
    pub favorites: String,
    pub command_installation_directory: &'static str,
    pub license: &'static str,
    pub log_directory: &'static str,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    configuration_file: "/etc/cmdcreate.toml",
    favorites: format!("{MAIN_PATH}/favorites"),
    command_installation_directory: "/usr/local/bin/",
    license: if get_distro_base() == DistroBase::Debian {
        // Because different distros just HAVE to have different paths for some bullshit reason
        "/usr/share/doc/cmdcreate/copyright/LICENSE"
    } else {
        "/usr/share/licenses/cmdcreate/LICENSE"
    },
    log_directory: "/tmp/cmdcreate-logs/",
});
