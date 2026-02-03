use std::{
    fs::{File, OpenOptions, create_dir_all, read_to_string, remove_dir_all, remove_file},
    io::Write as _,
    path::Path,
    sync::LazyLock,
};

use crate::{
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
    configs: format!("{}/.config/cmdcreate/config.toml", VARS.home),
    favorites: format!("{}/favorites", *MAIN_PATH),
    install_dir: format!("{}/files/", *MAIN_PATH),
    license: format!("{}/LICENSE.md", *MAIN_PATH),
    log_dir: format!("{}/logs", *MAIN_PATH),
});

pub fn init_fs_layout() {
    create_folder(&MAIN_PATH);
    create_folder(&PATHS.log_dir);
    create_folder(&PATHS.install_dir);

    create_file(&PATHS.favorites);

    log("utils/fs::init_fs_layout(): Filesystem initialized", 0);
}

pub fn init_git_fs() {
    log("utils/fs::init_git_fs(): Syncing offline files...", 0);

    retrieve_git_file(&PATHS.license, "LICENSE.md");
    retrieve_git_file(&PATHS.changelog, "changes.md");

    log("utils/fs::init_git_fs(): Offline files synced", 0);
}

pub fn retrieve_git_file(dest: &str, file_path: &str) {
    run_shell_command(&format!(
        "curl -sSo {dest} https://raw.githubusercontent.com/owen-debiasio/cmdcreate/master/{file_path}"
    ));
}

pub fn read_file_to_string(file_path: &str) -> String {
    read_to_string(file_path).unwrap_or_else(|e| {
        error("Error reading file:", &format!("\"{file_path}\": {e}"));
    })
}

pub fn overwrite_file(path: &str, contents: &str) {
    if let Some(file) = Path::new(path).parent()
        && let Err(e) = create_dir_all(file)
    {
        error(
            "Failed to overwrite file:",
            &format!("\"{:?}\": {e}", file.display()),
        );
    }

    if let Err(e) = File::create(path).and_then(|mut f| f.write_all(contents.as_bytes())) {
        error(&format!("Failed to overwrite {path}:"), &e.to_string());
    }
}

pub fn write_to_file(path: &str, contents: &str, append: bool) {
    if let Some(file) = Path::new(path).parent()
        && let Err(e) = create_dir_all(file)
    {
        error(
            "Failed to write to file:",
            &format!("\"{:?}\": {e}", file.display()),
        );
    }

    let mut opts = OpenOptions::new();
    opts.create(true).write(true);

    if append {
        opts.append(true);
    } else {
        opts.truncate(true);
    }

    if let Err(err) = opts
        .open(path)
        .and_then(|mut file| file.write_all(contents.as_bytes()))
    {
        error(&format!("Failed writing {path}:"), &err.to_string());
    }
}

pub fn remove_from_file(path: &str, contents: &str) {
    overwrite_file(
        path,
        &read_file_to_string(path)
            .lines()
            .filter(|line| line.trim() != contents)
            .fold(String::new(), |acc, line| format!("{acc}{line}\n")),
    );
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn create_folder(path: &str) {
    if let Err(e) = create_dir_all(path) {
        error(&format!("Failed to create folder {path}:"), &e.to_string());
    }
}

pub fn create_file(path: &str) {
    if let Some(parent) = Path::new(path).parent()
        && let Err(e) = create_dir_all(parent)
    {
        error(
            &format!("Failed to create parent folder for {path}:"),
            &e.to_string(),
        );
    }

    if !path_exists(path)
        && let Err(e) = File::create(path)
    {
        error(&format!("Failed to create file {path}:"), &e.to_string());
    }
}

pub fn delete_file(path: &str) {
    if path_exists(path)
        && let Err(e) = remove_file(path)
    {
        error(&format!("Failed to delete file {path}:"), &e.to_string());
    }
}

pub fn delete_folder(path: &str) {
    if path_exists(path)
        && let Err(e) = remove_dir_all(path)
    {
        error(&format!("Failed to delete folder {path}:"), &e.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::temp_dir, fs::write, path::PathBuf};

    fn test_dir(name: &str) -> PathBuf {
        let dir = temp_dir().join(name);

        let _ = remove_dir_all(&dir);

        create_dir_all(&dir).expect("Failed to create test directory");

        dir
    }

    #[test]
    fn create_folder_creates_directory() {
        let dir = test_dir("cmdcreate_create_folder").join("subdir");

        create_folder(dir.to_str().unwrap());
        assert!(dir.exists());
    }

    #[test]
    fn create_file_creates_file() {
        let file = test_dir("cmdcreate_create_file").join("file.txt");

        create_file(file.to_str().unwrap());
        assert!(file.exists());
    }

    #[test]
    fn write_to_file_overwrites() {
        let file = test_dir("cmdcreate_write_file").join("file.txt");

        write_to_file(file.to_str().unwrap(), "hello", false);

        assert_eq!(read_file_to_string(file.to_str().unwrap()), "hello");
    }

    #[test]
    fn write_to_file_appends() {
        let file = test_dir("cmdcreate_append_file").join("file.txt");

        write_to_file(file.to_str().unwrap(), "a\n", false);
        write_to_file(file.to_str().unwrap(), "b\n", true);

        let content = read_file_to_string(file.to_str().unwrap());

        assert!(content.contains('a'));
        assert!(content.contains('b'));
    }

    #[test]
    fn remove_from_file_removes_line() {
        let file = test_dir("cmdcreate_remove_from_file").join("file.txt");

        write_to_file(file.to_str().unwrap(), "one\ntwo\nthree\n", false);

        remove_from_file(file.to_str().unwrap(), "two");

        assert!(!read_file_to_string(file.to_str().unwrap()).contains("two"));
    }

    #[test]
    fn delete_file_removes_file() {
        let file = test_dir("cmdcreate_delete_file").join("file.txt");

        create_file(file.to_str().unwrap());
        delete_file(file.to_str().unwrap());

        assert!(!file.exists());
    }

    #[test]
    fn delete_folder_removes_directory() {
        let dir = test_dir("cmdcreate_delete_folder").join("dir");

        create_dir_all(&dir).unwrap();

        delete_folder(dir.to_str().unwrap());

        assert!(!dir.exists());
    }

    #[test]
    fn path_exists_reports_correctly() {
        let file = test_dir("cmdcreate_exists").join("exists.txt");

        assert!(!path_exists(file.to_str().unwrap()));
        write(&file, "hi").unwrap();

        assert!(path_exists(file.to_str().unwrap()));
    }
}
