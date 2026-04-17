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

//! This library just provides wrappers to already existing functions!
//! They are technically useless, but they make more sense to me.
//! If using filesystem actions, please refer to here.

use crate::{
    logger::{Severity, log},
    meta::project_information::PROJECT,
    utils::{
        colors::COLORS,
        io::error,
        net::not_connected_to_internet,
        sys::{
            command::{run_shell_command, system_command_is_installed},
            distro::{DistroBase, get_distro_base},
        },
    },
};
use anyhow::{Context, Result};
use std::{
    fs::{File, OpenOptions, create_dir_all, read_to_string, remove_dir_all, remove_file},
    io::Write as _,
    path::Path,
    string::ToString,
    sync::LazyLock,
};

pub static MAIN_PATH: &str = "/root/.local/share/cmdcreate";

pub struct Paths {
    pub configuration_file: &'static str,
    pub favorites: String,
    pub command_installation_directory: &'static str,
    pub license: &'static str,
    pub log_directory: String,
}

pub static PATHS: LazyLock<Paths> = LazyLock::new(|| Paths {
    configuration_file: "/etc/cmdcreate.toml",
    favorites: format!("{MAIN_PATH}/favorites"),
    command_installation_directory: "/usr/local/bin/",
    license: if get_distro_base() == DistroBase::Debian {
        // Because different distros just HAVE to have different paths for some bullshit reason
        "/usr/share/doc/cmdcreate/copyright/LICENSE"
    } else {
        "/usr/share/licenses/cmdcreate/LICENSE"
    },
    log_directory: format!("{MAIN_PATH}/logs"),
});

pub fn install_binary(mode: &str, binary: &str, destination: &str) {
    log(
        &format!(
            "
            utils/fs::install_binary(): \
            Installing binary \"{binary}\" \
            to \"{destination}\" \
            using mode \"{mode}\"..."
        ),
        Severity::Normal,
    );

    run_shell_command(&format!("install {mode} {binary} {destination}"));

    if !path_exists(destination) {
        error("Failed to install binary!", "Binary not found!")
    }

    log(
        "utils/fs::install_binary(): Successfully installed binary!",
        Severity::Normal,
    );
}

pub fn clone_repository(destination: &str) {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log(
        "utils/fs::clone_repository(): Cloning project repository...",
        Severity::Normal,
    );

    if !system_command_is_installed("git") {
        error(
            "Unable to clone repository.",
            "Please install git to continue.",
        )
    }

    println!("\n{blue}Cloning project repository...{reset}");

    let project_repo = PROJECT.repository;

    run_shell_command(&format!(
        "git clone --quiet --depth=1 \
        {project_repo}.git {destination}"
    ));

    if !path_exists(destination) {
        error("Failed to clone repository!", "Destination not found!")
    }

    log(
        &format!(
            "utils/fs::clone_repository(): \
            Successfully cloned repository \"{project_repo}\""
        ),
        Severity::Normal,
    );
}

pub fn download_file_to_location_via_curl(
    file_destination: &str,
    path_of_file_to_be_downloaded: &str,
) {
    if not_connected_to_internet() {
        error("Unable to retrieve file! You need internet first!", "")
    }

    run_shell_command(&format!(
        "curl -Lfo {file_destination} {path_of_file_to_be_downloaded}"
    ));

    if !path_exists(file_destination) {
        error("Downloaded file not found!", "Failed to download file!")
    }
}

pub fn init_fs_layout() -> Result<()> {
    create_folder(MAIN_PATH)?;
    create_folder(&PATHS.log_directory)?;
    create_folder(PATHS.command_installation_directory)?;
    create_file(&PATHS.favorites)?;
    create_file(PATHS.configuration_file)?;

    log(
        "utils/fs::init_fs_layout(): Filesystem initialized",
        Severity::Normal,
    );
    Ok(())
}

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

pub fn overwrite_file(
    path_to_file_to_overwrite: &str,
    contents_of_file_to_overwrite: &str,
) -> Result<()> {
    write_to_file(
        path_to_file_to_overwrite,
        contents_of_file_to_overwrite,
        false,
    )
}

pub fn write_to_file(
    path_to_file_to_be_written_to: &str,
    contents_to_write_to_file: &str,
    append_changes: bool,
) -> Result<()> {
    if let Some(parent_directory) = Path::new(path_to_file_to_be_written_to).parent() {
        create_dir_all(parent_directory).with_context(|| {
            format!("Failed to create parent directory for: {path_to_file_to_be_written_to}")
        })?;
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
        .with_context(|| format!("Failed to open file: {path_to_file_to_be_written_to}"))?;

    file_that_should_be_written_to
        .write_all(contents_to_write_to_file.as_bytes())
        .with_context(|| format!("Failed writing to file: {path_to_file_to_be_written_to}"))?;

    Ok(())
}

pub fn remove_from_file(path_to_file: &str, contents_to_remove: &str) -> Result<()> {
    overwrite_file(
        path_to_file,
        &read_file_to_string(path_to_file).replace(contents_to_remove, ""),
    )
}

pub fn path_exists(apparent_path: &str) -> bool {
    Path::new(apparent_path).exists()
}

pub fn create_folder(path_of_folder_to_create: &str) -> Result<()> {
    create_dir_all(path_of_folder_to_create)
        .with_context(|| format!("Failed to create folder: {path_of_folder_to_create}"))
}

pub fn create_file(file_to_be_created: &str) -> Result<()> {
    if let Some(parent_folder) = Path::new(file_to_be_created).parent() {
        create_dir_all(parent_folder)
            .with_context(|| format!("Failed to create parent folder for {file_to_be_created}"))?;
    }

    if !path_exists(file_to_be_created) {
        File::create(file_to_be_created)
            .with_context(|| format!("Failed to create file: {file_to_be_created}"))?;
    }

    Ok(())
}

pub fn delete_file(path_of_file_to_delete: &str) -> Result<()> {
    if path_exists(path_of_file_to_delete) {
        remove_file(path_of_file_to_delete)
            .with_context(|| format!("Failed to delete file: {path_of_file_to_delete}"))?;
    }

    Ok(())
}

pub fn delete_folder(path_of_folder_to_delete: &str) -> Result<()> {
    if path_exists(path_of_folder_to_delete) {
        remove_dir_all(path_of_folder_to_delete)
            .with_context(|| format!("Failed to delete folder: {path_of_folder_to_delete}"))?;
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
        let temp_directory = temp_dir().join(name);
        let _ = remove_dir_all(&temp_directory);

        create_dir_all(&temp_directory).expect("Failed to create test directory");

        temp_directory
    }

    #[test]
    fn create_folder_creates_directory() -> Result<()> {
        let temp_directory = test_dir("cmdcreate_create_folder").join("subdir");
        let path_of_temp_directory = temp_directory.to_string_lossy();

        create_folder(&path_of_temp_directory)?;

        assert!(temp_directory.exists());

        Ok(())
    }

    #[test]
    fn create_file_creates_file() -> Result<()> {
        let file_to_be_created = test_dir("cmdcreate_create_file").join("file.txt");
        let path_of_file = file_to_be_created.to_string_lossy();

        create_file(&path_of_file)?;

        assert!(file_to_be_created.exists());

        Ok(())
    }

    #[test]
    fn write_to_file_overwrites() -> Result<()> {
        let file_to_overwrite = test_dir("cmdcreate_write_file").join("file.txt");
        let path_of_file = file_to_overwrite.to_string_lossy();

        let sample_text = "this is a test";

        write_to_file(&path_of_file, sample_text, false)?;

        assert_eq!(read_file_to_string(&path_of_file), sample_text);

        Ok(())
    }

    #[test]
    fn write_to_file_appends() -> Result<()> {
        let file_to_append = test_dir("cmdcreate_append_file").join("file.txt");
        let path_of_appended_file = file_to_append.to_string_lossy();

        write_to_file(&path_of_appended_file, "a\n", false)?;
        write_to_file(&path_of_appended_file, "b\n", true)?;

        let appended_file_contents = read_file_to_string(&path_of_appended_file);

        assert!(appended_file_contents.contains('a'));
        assert!(appended_file_contents.contains('b'));

        Ok(())
    }

    #[test]
    fn remove_from_file_removes_line() -> Result<()> {
        let file_to_remove_contents = test_dir("cmdcreate_remove_from_file").join("file.txt");
        let path_of_that_file = file_to_remove_contents.to_string_lossy();

        write_to_file(&path_of_that_file, "one\ntwo\nthree\n", false)?;
        remove_from_file(&path_of_that_file, "two")?;

        let contents_of_that_file = read_file_to_string(&path_of_that_file);

        assert!(!contents_of_that_file.contains("two"));
        assert!(contents_of_that_file.contains("one"));

        Ok(())
    }

    #[test]
    fn delete_file_removes_file() -> Result<()> {
        let file_to_delete = test_dir("cmdcreate_delete_file").join("file.txt");
        let path_to_deleted_file = file_to_delete.to_string_lossy();

        create_file(&path_to_deleted_file)?;
        delete_file(&path_to_deleted_file)?;

        assert!(!file_to_delete.exists());

        Ok(())
    }

    #[test]
    fn delete_folder_removes_directory() -> Result<()> {
        let directory_to_delete = test_dir("cmdcreate_delete_folder").join("dir");
        let path_to_deleted_directory = directory_to_delete.to_string_lossy();

        create_dir_all(&directory_to_delete)?;
        delete_folder(&path_to_deleted_directory)?;

        assert!(!directory_to_delete.exists());

        Ok(())
    }

    #[test]
    fn path_exists_reports_correctly() -> Result<()> {
        let path_to_verify = test_dir("cmdcreate_exists").join("exists.txt");
        let path_of_the_path_that_needs_to_be_verified = path_to_verify.to_string_lossy();

        let sample_text = "this is a test";

        assert!(!path_exists(&path_of_the_path_that_needs_to_be_verified));
        write(&path_to_verify, sample_text)?;

        assert!(path_exists(&path_of_the_path_that_needs_to_be_verified));

        Ok(())
    }
}
