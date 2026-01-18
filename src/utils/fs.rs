use std::{
    fs::{File, OpenOptions, create_dir_all, read_to_string, remove_dir_all, remove_file},
    io::Write,
    path::Path,
    sync::LazyLock,
};

use crate::{
    configs::init_configs,
    logger::log,
    utils::{
        io::error,
        sys::{VARS, run_shell_command},
    },
};

pub static MAIN_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/.local/share/cmdcreate", VARS.home));

pub struct Paths {
    pub changelog: String,
    pub configs: String,
    pub favorites: String,
    pub install_dir: String,
    pub license: String,
    pub log_dir: String,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    changelog: format!("{}/changes.md", *MAIN_PATH),
    configs: format!("{}/.config/cmdcreate/", VARS.home),
    favorites: format!("{}/favorites", *MAIN_PATH),
    install_dir: format!("{}/files/", *MAIN_PATH),
    license: format!("{}/LICENSE", *MAIN_PATH),
    log_dir: format!("{}/logs/", *MAIN_PATH),
});

pub fn init_fs() {
    create_folder(&PATHS.log_dir);

    log(
        "utils/fs::init_fs(): Initializing cmdcreate's filesystem",
        0,
    );

    create_folder(&MAIN_PATH);
    create_folder(&PATHS.install_dir);
    create_file(&PATHS.favorites);

    init_configs();

    log("utils/fs::init_fs(): Initialized cmdcreate's filesystem", 0);
}

pub fn init_git_fs() {
    log("utils/fs::init_git_fs(): Initializing offline files", 0);

    retrieve_git_file(&PATHS.license, "LICENSE");
    retrieve_git_file(&PATHS.changelog, "changes.md");

    log("utils/fs::init_git_fs(): Initialized offline files", 0);
}

pub fn retrieve_git_file(dest: &str, file_path: &str) {
    log(
        &format!("utils/fs::retrieve_git_file(): Downloading file: {file_path} to {dest}"),
        0,
    );

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
    if let Some(parent) = Path::new(path).parent() {
        let _ = create_dir_all(parent);
    }

    if let Err(e) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| f.write_all(contents.as_bytes()))
    {
        panic!("Failed to write to {path}: {e}");
    }
}

pub fn overwrite_file(path: &str, contents: &str) {
    log(
        &format!("utils/fs::overwrite_file(): Overwriting file: {path} with: {contents}"),
        0,
    );

    if let Some(parent) = Path::new(path).parent() {
        let _ = create_dir_all(parent);
    }

    if let Err(e) = File::create(path).and_then(|mut f| f.write_all(contents.as_bytes())) {
        panic!("Failed to overwrite {path}: {e}");
    }
}

pub fn remove_from_file(path: &str, contents: &str) {
    log(
        &format!("utils/fs::remove_from_file(): Removing contents: \"{contents}\" from {path}"),
        0,
    );

    overwrite_file(
        path,
        &read_file_to_string(path)
            .lines()
            .filter(|line| line.trim() != contents)
            .fold(String::new(), |acc, line| format!("{acc}{line}\n")),
    );
}

pub fn path_exists(path: &str) -> bool {
    log(
        &format!("utils/fs::path_exists(): Determining if path \"{path}\" exists..."),
        0,
    );

    Path::new(path).exists()
}

pub fn create_folder(path: &str) {
    //log(
    // &format!("utils/fs::create_folder(): Creating folder: \"{path}\""),
    // 0,
    //);

    if let Err(e) = create_dir_all(path) {
        error(
            &format!("Failed to create folder: \"{path}\":"),
            &e.to_string(),
        );
    }
}

pub fn create_file(path: &str) {
    log(
        &format!("utils/fs::create_file(): Creating file: \"{path}\""),
        0,
    );

    if let Some(parent) = Path::new(path).parent()
        && let Err(e) = create_dir_all(parent)
    {
        error(
            &format!("Failed to create parent folder: \"{path}\":"),
            &e.to_string(),
        );
        return;
    }

    if !Path::new(path).exists()
        && let Err(e) = File::create(path)
    {
        error(
            &format!("Failed to create file: \"{path}\""),
            &e.to_string(),
        );
    }
}

pub fn delete_file(path: &str) {
    log(
        &format!("utils/fs::delete_file(): Deleting file: \"{path}\""),
        0,
    );

    if path_exists(path)
        && let Err(e) = remove_file(path)
    {
        error(&format!("Failed to delete file {path}:"), &e.to_string());
    }
}

pub fn delete_folder(path: &str) {
    log(
        &format!("utils/fs::delete_folder(): Deleting folder: \"{path}\""),
        0,
    );

    if path_exists(path)
        && let Err(e) = remove_dir_all(path)
    {
        error(&format!("Failed to delete folder {path}:"), &e.to_string());
    }
}
