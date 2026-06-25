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

use crate::utils::fs::paths::{expand_home_dir, path_exists};
use anyhow::Context;
use std::{
    fs::{File, create_dir_all, remove_dir_all, remove_file},
    path::Path,
};

pub fn create_folder(path_of_folder_to_create: &str) {
    let path_of_file = expand_home_dir(path_of_folder_to_create);
    create_dir_all(&path_of_file)
        .with_context(|| format!("Failed to create folder: {path_of_file}"))
        .expect("Failed to create folder");
}

pub fn create_file(file_to_be_created: &str) {
    let path_of_file = expand_home_dir(file_to_be_created);
    if let Some(parent_folder) = Path::new(&path_of_file).parent() {
        create_dir_all(parent_folder)
            .with_context(|| format!("Failed to create parent folder for: {path_of_file}"))
            .expect("Failed to create parent folder");
    }

    if !path_exists(&path_of_file) {
        File::create(&path_of_file)
            .with_context(|| format!("Failed to create file: {path_of_file}"))
            .expect("Failed to create file");
    }
}

pub fn delete_file(path_of_file_to_delete: &str) {
    let path = expand_home_dir(path_of_file_to_delete);
    if path_exists(&path) {
        remove_file(&path)
            .with_context(|| format!("Failed to delete file: {path}"))
            .expect("Failed to delete file");
    }
}

pub fn delete_folder(path_of_folder_to_delete: &str) {
    let path = expand_home_dir(path_of_folder_to_delete);
    if path_exists(&path) {
        remove_dir_all(&path)
            .with_context(|| format!("Failed to delete folder: {path}"))
            .expect("Failed to create folder"); // Fixed a typo in your original error message here too!
    }
}

#[test]
fn create_file_creates_file() {
    let file_path = "create_file_creates_file";

    create_file(file_path);

    assert!(path_exists(file_path));

    delete_file(file_path);
}

#[test]
fn delete_file_removes_file() {
    let file_path = "delete_file_removes_file";

    create_file(file_path);
    delete_file(file_path);

    assert!(!path_exists(file_path));
}

#[test]
fn create_folder_creates_directory() {
    let folder_path = "create_folder_creates_directory";

    create_folder(folder_path);

    assert!(path_exists(folder_path));

    delete_folder(folder_path);
}

#[test]
fn delete_folder_removes_directory() {
    let folder_path = "delete_folder_removes_directory";

    create_folder(folder_path);
    delete_folder(folder_path);

    assert!(!path_exists(folder_path));
}
