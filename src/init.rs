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
    VERSION,
    configs::init_configs,
    logger::log,
    utils::{
        fs::init_fs_layout,
        io::error,
        net::is_offline,
        sys::{ARCH, VARS, get_distro_base, installation_method, is_root},
    },
    version::is_development_version,
};

pub fn debug_intro() -> String {
    format!(
        "                               ----------------
        Welcome to cmdcreate!           Version: {VERSION} {}
            Created by:                 CPU Architecture: {ARCH}
           Owen Debiasio                Distro Base: {:?}
       owen.debiasio@gmail.com          Preferred installation method: {:?}
                                        Internet status: {}
    Have an issue? Copy this text       Preferred text editor: {}
          and open an issue             Shell in use: {}
                                        ----------------",
        if is_development_version() {
            "(devel)"
        } else {
            "(stable)"
        },
        get_distro_base(),
        installation_method(),
        if is_offline() { "offline" } else { "connected" },
        VARS.editor,
        VARS.shell,
    )
}

pub fn init() {
    if !is_root() {
        error("Please run cmdcreate as root.", "")
    }

    init_fs_layout().expect("Failed to initialize filesystem");
    init_configs();

    log(
        &format!(
            "init::init(): Starting cmdcreate...\n         {}",
            debug_intro()
        ),
        0,
    );
}
