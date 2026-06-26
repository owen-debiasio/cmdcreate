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

use cmdcreate::{
    commands::core::{
        create::{NEW_COMMAND_HEADER, create},
        remove::remove,
    },
    utils::fs::{
        core::read_write::read_file_to_string,
        paths::{PATHS, path_exists},
    },
};

static SAMPLE_COMMAND_CONTENTS: &str = "echo hi";

struct TestCommand;

impl TestCommand {
    fn create(command_name: &str) {
        create(command_name, SAMPLE_COMMAND_CONTENTS, false);
    }

    fn remove(command_name: &str) {
        remove(command_name, true);
    }

    fn get_install_path(command_name: &str) -> String {
        let installation_path = PATHS.command_installation_directory;
        format!("{installation_path}{command_name}")
    }
}

#[test]
fn command_file_exists() {
    let test_command_name = "command_file_exists";
    TestCommand::create(test_command_name);

    let command_install_path = &TestCommand::get_install_path(test_command_name);

    assert!(path_exists(command_install_path));

    TestCommand::remove(test_command_name);
}

#[test]
fn command_contains_contents() {
    let test_command_name = "command_contains_contents";
    TestCommand::create(test_command_name);

    let command_install_path = &TestCommand::get_install_path(test_command_name);

    let command_contents = read_file_to_string(command_install_path);

    assert_eq!(
        command_contents.replace(NEW_COMMAND_HEADER, "").trim(),
        SAMPLE_COMMAND_CONTENTS
    );

    TestCommand::remove(test_command_name);
}
