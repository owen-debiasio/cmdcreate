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

use crate::utils::io::error;

use anyhow::Context;
use std::{
    fs::{File, OpenOptions, create_dir_all, read_to_string, remove_dir_all, remove_file},
    io::Write,
    path::Path,
};

pub fn read_file_to_string(path_to_file_to_read: &str) -> String {
    read_to_string(path_to_file_to_read)
        .unwrap_or_else(|err| {
            error(
                &format!("Failed reading file: {path_to_file_to_read}"),
                &err.to_string(),
            );
        })
        .replace("\r\n", "\n") // Avoid CRLF encoding (because it screws shit up)
}

pub fn overwrite_file(path_to_file_to_overwrite: &str, contents_of_file_to_overwrite: &str) {
    write_to_file(
        path_to_file_to_overwrite,
        contents_of_file_to_overwrite,
        false,
    );
}

pub fn write_to_file(
    path_to_file_to_be_written_to: &str,
    contents_to_write_to_file: &str,
    append_changes: bool,
) {
    if let Some(parent_directory) = Path::new(path_to_file_to_be_written_to).parent() {
        create_dir_all(parent_directory)
            .with_context(|| {
                format!(
                    "
                Failed to create parent directory for: \
                {path_to_file_to_be_written_to}"
                )
            })
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
        .open(path_to_file_to_be_written_to)
        .with_context(|| {
            format!(
                "
            Failed to open file: \
            {path_to_file_to_be_written_to}"
            )
        })
        .expect("Failed to get file");

    file_that_should_be_written_to
        .write_all(contents_to_write_to_file.as_bytes())
        .with_context(|| {
            format!(
                "
            Failed writing to file: \
            {path_to_file_to_be_written_to}"
            )
        })
        .expect("Failed to write contents");
}

pub fn remove_from_file(path_to_file: &str, contents_to_remove: &str) {
    overwrite_file(
        path_to_file,
        &read_file_to_string(path_to_file).replace(contents_to_remove, ""),
    );
}

pub fn path_exists(apparent_path: &str) -> bool {
    Path::new(apparent_path).exists()
}

pub fn create_folder(path_of_folder_to_create: &str) {
    create_dir_all(path_of_folder_to_create)
        .with_context(|| {
            format!(
                "
            Failed to create folder: \
            {path_of_folder_to_create}"
            )
        })
        .expect("Failed to create folder");
}

pub fn create_file(file_to_be_created: &str) {
    if let Some(parent_folder) = Path::new(file_to_be_created).parent() {
        create_dir_all(parent_folder)
            .with_context(|| {
                format!(
                    "
                Failed to create parent folder for: \
                {file_to_be_created}"
                )
            })
            .expect("Failed to create parent folder");
    }

    if !path_exists(file_to_be_created) {
        File::create(file_to_be_created)
            .with_context(|| {
                format!(
                    "
                Failed to create file: \
                {file_to_be_created}"
                )
            })
            .expect("Failed to create file");
    }
}

pub fn delete_file(path_of_file_to_delete: &str) {
    if path_exists(path_of_file_to_delete) {
        remove_file(path_of_file_to_delete)
            .with_context(|| {
                format!(
                    "
                Failed to delete file: \
                {path_of_file_to_delete}"
                )
            })
            .expect("Failed to delete file");
    }
}

pub fn delete_folder(path_of_folder_to_delete: &str) {
    if path_exists(path_of_folder_to_delete) {
        remove_dir_all(path_of_folder_to_delete)
            .with_context(|| {
                format!(
                    "
                Failed to delete folder: \
                {path_of_folder_to_delete}"
                )
            })
            .expect("Failed to create folder");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{
        env::temp_dir,
        fs::{create_dir_all, remove_dir_all, write},
        path::PathBuf,
    };

    fn test_dir(name: &str) -> PathBuf {
        let temp_directory = temp_dir().join(name);
        let _ = remove_dir_all(&temp_directory);

        create_dir_all(&temp_directory).expect("Failed to create test directory");

        temp_directory
    }

    #[test]
    fn create_folder_creates_directory() {
        let temp_directory = test_dir("cmdcreate_create_folder").join("subdir");
        let path_of_temp_directory = temp_directory.to_string_lossy();

        create_folder(&path_of_temp_directory);

        assert!(temp_directory.exists());
    }

    #[test]
    fn create_file_creates_file() {
        let file_to_be_created = test_dir("cmdcreate_create_file").join("file.txt");
        let path_of_file = file_to_be_created.to_string_lossy();

        create_file(&path_of_file);

        assert!(file_to_be_created.exists());
    }

    #[test]
    fn write_to_file_overwrites() {
        let file_to_overwrite = test_dir("cmdcreate_write_file").join("file.txt");
        let path_of_file = file_to_overwrite.to_string_lossy();

        let sample_text = "this is a test";

        write_to_file(&path_of_file, sample_text, false);

        assert_eq!(read_file_to_string(&path_of_file), sample_text);
    }

    #[test]
    fn write_to_file_appends() {
        let file_to_append = test_dir("cmdcreate_append_file").join("file.txt");
        let path_of_appended_file = file_to_append.to_string_lossy();

        write_to_file(&path_of_appended_file, "a\n", false);
        write_to_file(&path_of_appended_file, "b\n", true);

        let appended_file_contents = read_file_to_string(&path_of_appended_file);

        assert!(appended_file_contents.contains('a'));
        assert!(appended_file_contents.contains('b'));
    }

    #[test]
    fn remove_from_file_removes_line() {
        let file_to_remove_contents = test_dir("cmdcreate_remove_from_file").join("file.txt");
        let path_of_that_file = file_to_remove_contents.to_string_lossy();

        write_to_file(&path_of_that_file, "one\ntwo\nthree\n", false);
        remove_from_file(&path_of_that_file, "two");

        let contents_of_that_file = read_file_to_string(&path_of_that_file);

        assert!(!contents_of_that_file.contains("two"));
        assert!(contents_of_that_file.contains("one"));
    }

    #[test]
    fn delete_file_removes_file() {
        let file_to_delete = test_dir("cmdcreate_delete_file").join("file.txt");
        let path_to_deleted_file = file_to_delete.to_string_lossy();

        create_file(&path_to_deleted_file);
        delete_file(&path_to_deleted_file);

        assert!(!file_to_delete.exists());
    }

    #[test]
    fn delete_folder_removes_directory() {
        let directory_to_delete = test_dir("cmdcreate_delete_folder").join("dir");
        let path_to_deleted_directory = directory_to_delete.to_string_lossy();

        create_dir_all(&directory_to_delete).expect("Failed to create directory");
        delete_folder(&path_to_deleted_directory);

        assert!(!directory_to_delete.exists());
    }

    #[test]
    fn path_exists_reports_correctly() {
        let path_to_verify = test_dir("cmdcreate_exists").join("exists.txt");
        let path_of_the_path_that_needs_to_be_verified = path_to_verify.to_string_lossy();

        let sample_text = "this is a test";

        assert!(!path_exists(&path_of_the_path_that_needs_to_be_verified));
        write(&path_to_verify, sample_text).expect("Failed to write file");

        assert!(path_exists(&path_of_the_path_that_needs_to_be_verified));
    }
}
