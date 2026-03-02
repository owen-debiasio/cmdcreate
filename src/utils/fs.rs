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

use crate::{
    logger::log,
    utils::{
        net::is_offline,
        sys::{DistroBase, VARS, get_distro_base, run_shell_command},
    },
};
use anyhow::{Context, Result, anyhow};
use std::{
    fs::{self, File, OpenOptions, create_dir_all, read_to_string},
    io::Write as _,
    path::Path,
    string::ToString,
    sync::LazyLock,
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
    configs: "/etc/cmdcreate.toml".to_string(),
    favorites: format!("{}/favorites", *MAIN_PATH),
    install_dir: "/usr/local/bin/".to_string(),
    license: format!(
        "{:?}",
        if get_distro_base() == DistroBase::Debian || get_distro_base() == DistroBase::Unknown {
            "/usr/share/doc/cmdcreate/copyright"
        } else {
            "/usr/share/licenses/cmdcreate/LICENSE"
        }
    ),
    log_dir: format!("{}/logs", *MAIN_PATH),
});

pub fn init_fs_layout() -> Result<()> {
    create_folder(&MAIN_PATH)?;
    create_folder(&PATHS.log_dir)?;
    create_folder(&PATHS.install_dir)?;
    create_file(&PATHS.favorites)?;
    create_file(&PATHS.configs)?;

    log("utils/fs::init_fs_layout(): Filesystem initialized", 0);
    Ok(())
}

pub fn init_git_fs() -> Result<()> {
    log("utils/fs::init_git_fs(): Syncing offline files...", 0);

    if is_offline() {
        log(
            "utils/fs::init_git_fs(): No internet skipping downloading offline files...",
            1,
        );
        return Ok(());
    }
    retrieve_git_file(&PATHS.changelog, "changes.md")?;

    log("utils/fs::init_git_fs(): Offline files synced", 0);
    Ok(())
}

pub fn retrieve_git_file(dest: &str, file_path: &str) -> Result<()> {
    if is_offline() {
        return Err(anyhow!("Cannot sync {file_path}: System is offline"));
    }

    run_shell_command(&format!(
        "curl -sSo {dest} https://raw.githubusercontent.com/owen-debiasio/cmdcreate/master/{file_path}"
    ));

    Ok(())
}

pub fn read_file_to_string(file_path: &str) -> String {
    read_to_string(file_path).unwrap()
}

pub fn overwrite_file(path: &str, contents: &str) -> Result<()> {
    write_to_file(path, contents, false)
}

pub fn write_to_file(path: &str, contents: &str, append: bool) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory for: {path}"))?;
    }

    let mut opts = OpenOptions::new();
    opts.create(true).write(true);

    if append {
        opts.append(true);
    } else {
        opts.truncate(true);
    }

    let mut file = opts
        .open(path)
        .with_context(|| format!("Failed to open file: {path}"))?;

    file.write_all(contents.as_bytes())
        .with_context(|| format!("Failed writing to file: {path}"))?;

    Ok(())
}

pub fn remove_from_file(path: &str, contents: &str) -> Result<()> {
    overwrite_file(path, &read_file_to_string(path).replace(contents, ""))
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn create_folder(path: &str) -> Result<()> {
    create_dir_all(path).with_context(|| format!("Failed to create folder: {path}"))
}

pub fn create_file(path: &str) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        create_dir_all(parent)
            .with_context(|| format!("Failed to create parent folder for {path}"))?;
    }

    if !path_exists(path) {
        File::create(path).with_context(|| format!("Failed to create file: {path}"))?;
    }
    Ok(())
}

pub fn delete_file(path: &str) -> Result<()> {
    if path_exists(path) {
        fs::remove_file(path).with_context(|| format!("Failed to delete file: {path}"))?;
    }
    Ok(())
}

pub fn delete_folder(path: &str) -> Result<()> {
    if path_exists(path) {
        fs::remove_dir_all(path).with_context(|| format!("Failed to delete folder: {path}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::{
        env::temp_dir,
        fs::{create_dir_all, remove_dir_all, write},
        path::PathBuf,
    };

    fn test_dir(name: &str) -> PathBuf {
        let dir = temp_dir().join(name);
        let _ = remove_dir_all(&dir);
        create_dir_all(&dir).expect("Failed to create test directory");
        dir
    }

    #[test]
    fn create_folder_creates_directory() -> Result<()> {
        let dir = test_dir("cmdcreate_create_folder").join("subdir");
        let path_str = dir.to_string_lossy();

        create_folder(&path_str)?;
        assert!(dir.exists());
        Ok(())
    }

    #[test]
    fn create_file_creates_file() -> Result<()> {
        let file = test_dir("cmdcreate_create_file").join("file.txt");
        let path_str = file.to_string_lossy();

        create_file(&path_str)?;
        assert!(file.exists());
        Ok(())
    }

    #[test]
    fn write_to_file_overwrites() -> Result<()> {
        let file = test_dir("cmdcreate_write_file").join("file.txt");
        let path_str = file.to_string_lossy();

        write_to_file(&path_str, "hello", false)?;

        assert_eq!(read_file_to_string(&path_str), "hello");
        Ok(())
    }

    #[test]
    fn write_to_file_appends() -> Result<()> {
        let file = test_dir("cmdcreate_append_file").join("file.txt");
        let path_str = file.to_string_lossy();

        write_to_file(&path_str, "a\n", false)?;
        write_to_file(&path_str, "b\n", true)?;

        let content = read_file_to_string(&path_str);

        assert!(content.contains('a'));
        assert!(content.contains('b'));
        Ok(())
    }

    #[test]
    fn remove_from_file_removes_line() -> Result<()> {
        let file = test_dir("cmdcreate_remove_from_file").join("file.txt");
        let path_str = file.to_string_lossy();

        write_to_file(&path_str, "one\ntwo\nthree\n", false)?;
        remove_from_file(&path_str, "two")?;

        let content = read_file_to_string(&path_str);
        assert!(!content.contains("two"));
        assert!(content.contains("one"));
        Ok(())
    }

    #[test]
    fn delete_file_removes_file() -> Result<()> {
        let file = test_dir("cmdcreate_delete_file").join("file.txt");
        let path_str = file.to_string_lossy();

        create_file(&path_str)?;
        delete_file(&path_str)?;

        assert!(!file.exists());
        Ok(())
    }

    #[test]
    fn delete_folder_removes_directory() -> Result<()> {
        let dir = test_dir("cmdcreate_delete_folder").join("dir");
        let path_str = dir.to_string_lossy();

        create_dir_all(&dir)?;
        delete_folder(&path_str)?;

        assert!(!dir.exists());
        Ok(())
    }

    #[test]
    fn path_exists_reports_correctly() -> Result<()> {
        let file = test_dir("cmdcreate_exists").join("exists.txt");
        let path_str = file.to_string_lossy();

        assert!(!path_exists(&path_str));
        write(&file, "hi")?;

        assert!(path_exists(&path_str));
        Ok(())
    }
}
