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
    core::logger::{consts::Severity, main::log},
    run_shell_command,
    utils::{
        fs::{
            core::{
                creation::{create_file, create_folder},
                read_write::{read_file_to_string, write_to_file},
            },
            paths::{MAIN_PATH, PATHS, path_exists},
        },
        io::error,
        sys::env::{ENVIRONMENT_VARIABLES, running_as_root},
    },
};

pub fn init_filesystem() {
    log(
        "utils::fs::init::init_filesystem(): Initializing filesystem...",
        Severity::Normal,
    );

    create_cmdcreate_filesystem();

    // Add `~/.local/bin/cmdcreate` to PATH
    if !running_as_root() {
        add_home_install_directory_to_path();
    }

    log(
        "utils::fs::init::init_filesystem(): Filesystem initialized",
        Severity::Normal,
    );
}

fn create_cmdcreate_filesystem() {
    let favorites_file = &PATHS.favorites;
    let config_file = PATHS.configuration_file;
    let command_install_dir = PATHS.command_installation_directory;
    let log_directory = PATHS.log_directory;

    let essential_folders_exist = path_exists(favorites_file)
        && path_exists(config_file)
        && path_exists(&MAIN_PATH)
        && path_exists(log_directory);

    if essential_folders_exist {
        return;
    }

    create_folder(&MAIN_PATH);

    create_folder(log_directory);

    if !running_as_root() {
        create_folder(command_install_dir);
    }

    create_file(favorites_file);
    create_file(config_file);

    // Fix issues with running cmdcreate not as root on some systems
    if !run_shell_command!(bool: "chmod -R 777 /tmp/cmdcreate-logs") {
        error(
            "Failed to mark log '/tmp/cmdcreate-logs' with 'chmod -R 777'.",
            None,
        )
    }

    if !essential_folders_exist && (!running_as_root() && !path_exists(command_install_dir)) {
        error(
            "Failed to initialize filesystem!",
            Some("Directories don't exist!"),
        )
    }
}

pub fn add_home_install_directory_to_path() {
    let shell = ENVIRONMENT_VARIABLES.shell.to_lowercase();

    // 1. Map directly to standard Linux shell configuration files
    let shellrc = if shell.contains("zsh") {
        "~/.zshrc"
    } else if shell.contains("fish") {
        "~/.config/fish/config.fish"
    } else if shell.contains("bash") {
        "~/.bashrc"
    } else if shell.contains("tcsh") || shell.contains("csh") {
        "~/.tcshrc"
    } else {
        "~/.profile"
    };

    log(
        &format!(
            "cmdcreate::utils::fs::init::add_home_install_directory_to_path(): Using shellrc file: {shellrc}"
        ),
        Severity::Normal,
    );

    let command_install_dir: String = PATHS.command_installation_directory.replace('~', "$HOME");

    let path_to_add = if shell.contains("fish") {
        format!("\n# Added by cmdcreate\nset -gx PATH {command_install_dir} $PATH\n")
    } else if shell.contains("tcsh") || shell.contains("csh") {
        format!("\n# Added by cmdcreate\nsetenv PATH \"{command_install_dir}:$PATH\"\n")
    } else {
        format!("\n# Added by cmdcreate\nexport PATH=\"{command_install_dir}:$PATH\"\n")
    };

    let path_to_add = path_to_add.replace("cmdcreate/", "cmdcreate");

    let shellrc_contents = read_file_to_string(shellrc);
    let shellrc_contains_path_define = shellrc_contents.contains(path_to_add.trim());

    if shellrc_contains_path_define {
        log(
            "cmdcreate::utils::fs::init::add_home_install_directory_to_path(): PATH exists in shellrc, skipping...",
            Severity::Normal,
        );
    } else {
        log(
            "cmdcreate::utils::fs::init::add_home_install_directory_to_path(): Writing PATH to shellrc...",
            Severity::Normal,
        );

        write_to_file(shellrc, &path_to_add, true);
    }

    log(
        "cmdcreate::utils::fs::init::add_home_install_directory_to_path(): Wrote PATH to shellrc.",
        Severity::Normal,
    );
}
