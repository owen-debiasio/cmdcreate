use std::{
    fs::{self, File, OpenOptions, create_dir_all, read_to_string, remove_file},
    io::Write,
    path::Path,
    sync::LazyLock,
};

use crate::utils::{
    msgs::error,
    sys::{VARS, run_shell_command},
};

pub fn get_files() {
    retrieve_git_file(&PATHS.license, "LICENSE");
    retrieve_git_file(&PATHS.changelog, "changes.md");
}

pub fn retrieve_git_file(dest: &str, file_path: &str) {
    run_shell_command(&format!(
        "curl -sSo {dest} https://raw.githubusercontent.com/owen-debiasio/cmdcreate/master/{file_path}"
    ));
}

pub fn read_file_to_string(file_path: &str) -> String {
    read_to_string(file_path).unwrap_or_else(|e| {
        error("Error reading file:", &format!("\"{file_path}\": {e}"));
        String::new()
    })
}

pub fn write_to_file(path: &str, contents: &str) {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Failed to open or create file")
        .write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

pub fn overwrite_file(path: &str, contents: &str) {
    File::create(path)
        .expect("Failed to overwrite file")
        .write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

pub fn remove_from_file(path: &str, target: &str) {
    let contents = read_file_to_string(path);

    let mut new_contents = String::with_capacity(contents.len());

    for line in contents.lines() {
        if line.trim() != target {
            new_contents.push_str(line);
            new_contents.push('\n');
        }
    }

    overwrite_file(path, &new_contents);
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn create_folder(path: &str) {
    match create_dir_all(path) {
        Ok(()) => {}
        Err(e) => error(
            &format!("Failed to create folder: \"{path}\":"),
            &e.to_string(),
        ),
    }
}

pub fn create_file(path: &str) {
    if let Some(parent) = Path::new(path).parent() {
        let _ = create_dir_all(parent);
    }

    if !Path::new(path).exists() {
        let _ = File::create(path);
    }
}

pub fn delete_file(path: &str) {
    if let Err(e) = remove_file(path) {
        error(&format!("Failed to delete file {path}:"), &e.to_string());
    }
}

pub fn delete_folder(path: &str) {
    if let Err(e) = fs::remove_dir_all(path) {
        error(&format!("Failed to delete folder {path}:"), &e.to_string());
    }
}

pub static MAIN_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/.local/share/cmdcreate", VARS.home));

pub struct Paths {
    pub favorites: String,

    pub install_dir: String,

    pub license: String,
    pub changelog: String,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    favorites: format!("{}/favorites", *MAIN_PATH),

    install_dir: format!("{}/files/", *MAIN_PATH),

    license: format!("{}/LICENSE", *MAIN_PATH),
    changelog: format!("{}/changes.md", *MAIN_PATH),
});
