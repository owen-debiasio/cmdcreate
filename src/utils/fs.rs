/// cmdcreate: Filesystem utilities
///
/// This module provides helper functions for reading, writing, creating, and deleting
/// files and folders. It handles errors gracefully using the cmdcreate error reporting
/// system.
///
/// # Features
/// - Read/write files with error handling
/// - Create folders and files, including parent directories
/// - Delete files and folders safely
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::utils::msgs::error;

/// Reads the contents of a file into a `String`
///
/// # Arguments
/// * `file_path` - Path to the file to read
///
/// # Returns
/// * `String` - File contents, or empty string if an error occurs
pub fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or_else(|e| {
        // Print error using cmdcreate's error system
        error("Error reading file:", &format!("\"{file_path}\": {e}"));
        String::new()
    })
}

/// Writes a string to a file, overwriting any existing contents
///
/// # Arguments
/// * `path` - Path to the file
/// * `contents` - String data to write
pub fn write_to_file(path: &str, contents: &str) {
    let mut file = OpenOptions::new()
        .create(true) // make the file if it doesn't exist
        .append(true) // write at the end instead of overwriting
        .open(path)
        .expect("Failed to open or create file");

    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

/// Overwrites a file with new contents
///
/// # Arguments
/// * `path` - Path to the file
/// * `contents` - String data to write
pub fn overwrite_file(path: &str, contents: &str) {
    let mut file = File::create(path).expect("Failed to overwrite file");
    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

/// Removes text from a file, overwriting any existing contents
///
/// # Arguments
/// * `path` - Path to the file
/// * `contents` - String data to remove
pub fn remove_from_file(path: &str, target: &str) {
    let contents = read_file_to_string(path);

    let new_contents = contents
        .lines()
        .filter(|line| line.trim() != target)
        .map(|line| format!("{line}\n"))
        .collect::<String>();

    overwrite_file(path, &new_contents);
}

/// Checks if a file/folder path exists at the given path
///
/// # Arguments
/// * `path` - File/folder path
///
/// # Returns
/// * `bool` - true if path exists, false otherwise
///
/// # Note
pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Creates a folder and all parent directories if they don't exist
///
/// # Arguments
/// * `path` - Folder path to create
pub fn _create_folder(path: &str) {
    match fs::create_dir_all(path) {
        Ok(_) => {} // Folder created successfully
        Err(e) => error(
            &format!("Failed to create folder: \"{path}\":"),
            &e.to_string(),
        ),
    }
}

/// Creates an empty file at the given path
///
/// Also creates parent directories if needed
///
/// # Arguments
/// * `path` - File path to create
pub fn create_file(path: &str) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }

    if !Path::new(path).exists() {
        let _ = fs::File::create(path);
    }
}

/// Deletes a file safely
///
/// # Arguments
/// * `path` - File path to delete
pub fn delete_file(path: &str) {
    if let Err(e) = fs::remove_file(path) {
        // Report error if deletion fails
        error(&format!("Failed to delete file {path}:"), &e.to_string());
    }
}

/// Deletes a folder and all its contents safely
///
/// # Arguments
/// * `path` - Folder path to delete
pub fn delete_folder(path: &str) {
    if let Err(e) = fs::remove_dir_all(path) {
        // Report error if deletion fails
        error(&format!("Failed to delete folder {path}:"), &e.to_string());
    }
}
