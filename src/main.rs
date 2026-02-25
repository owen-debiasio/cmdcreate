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

mod commands;
mod configs;
mod init;
mod logger;
mod parse;
mod usage;
mod utils;
mod version;

use crate::{
    init::init,
    logger::log,
    parse::parse,
    usage::cmdcreate_usage,
    utils::{fs::init_git_fs, sys::return_args},
    version::VERSION,
};

fn main() {
    init();

    log("main::main(): Retrieving args...", 0);

    let mut args = return_args();

    args.retain(|a| !matches!(a.as_str(), "-V" | "--verbose" | "-o" | "--offline"));

    if args.is_empty() {
        cmdcreate_usage();
    }

    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let mut i = 0;

    while let Some(cmd) = args.get(i).map(String::as_str) {
        if !cmd.starts_with('-') {
            break;
        }

        if matches!(cmd, "-V" | "--verbose") {
            i += 1;
            continue;
        }

        if matches!(
            cmd,
            "-l" | "--license" | "-c" | "--changelog" | "-g" | "--get_offline_files"
        ) {
            init_git_fs().expect("Failed to initialize git files");
        }

        parse(cmd, args);

        i += 1;
    }

    if let Some(cmd) = args.get(i).map(String::as_str) {
        parse(cmd, &args[i..]);
    }
}
