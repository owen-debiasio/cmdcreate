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

use std::{
    fs::{OpenOptions, create_dir_all, read_to_string},
    io::Write,
    path::Path,
};

use anyhow::Context;

use crate::utils::{
    fs::paths::{expand_home_dir, path_exists},
    io::error,
};

pub fn read_file_to_string(path_to_file_to_read: &str) -> String {
    let path = expand_home_dir(path_to_file_to_read);

    if !path_exists(&path) {
        return String::new();
    }

    read_to_string(&path)
        .unwrap_or_else(|err| {
            error(
                &format!("Failed reading file: {path}"),
                Some(&err.to_string()),
            );
        })
        .replace("\r\n", "\n")
}

pub fn overwrite_file(path_to_file_to_overwrite: &str, contents_of_file_to_overwrite: &str) {
    let path = expand_home_dir(path_to_file_to_overwrite);
    write_to_file(&path, contents_of_file_to_overwrite, false);
}

pub fn write_to_file(
    path_to_file_to_be_written_to: &str,
    contents_to_write_to_file: &str,
    append_changes: bool,
) {
    let path_of_file = expand_home_dir(path_to_file_to_be_written_to);
    if let Some(parent_directory) = Path::new(&path_of_file).parent() {
        create_dir_all(parent_directory)
            .with_context(|| format!("Failed to create parent directory for: {path_of_file}"))
            .expect("Failed to write to file");
    }

    let mut file_options = OpenOptions::new();
    file_options.create(true).write(true);

    if append_changes {
        file_options.append(true);
    } else {
        file_options.truncate(true);
    }

    let mut file_that_should_be_written_to = file_options
        .open(&path_of_file)
        .with_context(|| format!("Failed to open file: {path_of_file}"))
        .expect("Failed to get file");

    file_that_should_be_written_to
        .write_all(format!("{contents_to_write_to_file}\n").as_bytes())
        .with_context(|| format!("Failed writing to file: {path_of_file}"))
        .expect("Failed to write contents");
}

pub fn remove_from_file(path_to_file: &str, contents_to_remove: &str) {
    let path = expand_home_dir(path_to_file);
    overwrite_file(
        &path,
        &read_file_to_string(&path).replace(contents_to_remove, ""),
    );
}

#[test]
fn write_to_file_overwrites() {
    use crate::utils::fs::core::creation::{create_file, delete_file};

    let file_name = "write_to_file_overwrites";
    create_file(file_name);

    let sample_text = "this is a test";

    write_to_file(file_name, sample_text, false);

    assert_eq!(read_file_to_string(file_name), format!("{sample_text}\n"));

    delete_file(file_name);
}

#[test]
fn write_to_file_appends() {
    use crate::utils::fs::core::creation::{create_file, delete_file};

    let file_name = "write_to_file_appends";
    create_file(file_name);

    write_to_file(file_name, "a\n", false);
    write_to_file(file_name, "b\n", true);

    let appended_file_contents = read_file_to_string(file_name);

    assert!(appended_file_contents.contains('a'));
    assert!(appended_file_contents.contains('b'));

    delete_file(file_name);
}

#[test]
fn remove_from_file_removes_line() {
    use crate::utils::fs::core::creation::{create_file, delete_file};

    let file_name = "remove_from_file_removes_line";
    create_file(file_name);

    write_to_file(file_name, "one\ntwo\nthree\n", false);
    remove_from_file(file_name, "two");

    let contents_of_that_file = read_file_to_string(file_name);

    assert!(contents_of_that_file.contains("one"));
    assert!(!contents_of_that_file.contains("two"));

    delete_file(file_name);
}
