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
    commands::updater::main::update,
    meta::{author_information::AUTHOR, get_project_copyright_info, project_information::PROJECT},
    output,
    utils::{colors::COLORS, git::get_latest_tag, io::error, net::not_connected_to_internet},
};

use std::cmp::Ordering;

pub const CURRENT_PROJECT_VERSION: &str = "v1.3.4";

pub fn version_is_development_build() -> bool {
    let parse_version = |parsed_version_digits: &str| -> (u32, u32, u32) {
        // 'v' always comes before the version
        // '.' separates the version values

        let version_digits: Vec<u32> = parsed_version_digits
            .trim_start_matches('v')
            .split('.')
            .map(|version_digit_splitter| version_digit_splitter.parse().unwrap_or(0))
            .collect();
        (
            *version_digits.first().unwrap_or(&0),
            *version_digits.get(1).unwrap_or(&0),
            *version_digits.get(2).unwrap_or(&0),
        )
    };

    let latest_retrieved_tag = &get_latest_tag();
    let version_to_parse = &parse_version(latest_retrieved_tag);

    match parse_version(CURRENT_PROJECT_VERSION).cmp(version_to_parse) {
        Ordering::Less | Ordering::Equal => false,
        Ordering::Greater => true,
    }
}

pub fn get_build_status() -> &'static str {
    if version_is_development_build() {
        if not_connected_to_internet() {
            "(build status unknown)"
        } else {
            "(development)"
        }
    } else {
        "(stable)"
    }
}

pub fn print_version_info() {
    let project_name = PROJECT.name;

    let project_author = AUTHOR.name;
    let project_author_email = AUTHOR.email;

    let project_copyright_info = get_project_copyright_info();

    let build_status = get_build_status();

    output!(
        "
{project_name} {CURRENT_PROJECT_VERSION} {build_status}
Copyright (C) {project_copyright_info}.
License GPLv3+: GNU GPL version 3 or later <https://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by {project_author} <{project_author_email}>.
        ",
        true
    );
}

pub fn check() {
    let (blue, green, magenta) = (COLORS.blue, COLORS.green, COLORS.magenta);

    if not_connected_to_internet() {
        error(
            "You must have internet to continue with this operation!",
            None,
        )
    }

    output!("\nChecking for updates...", true);

    let latest_stable_version = get_latest_tag();

    if version_is_development_build() {
        output!(
            "\nYou are running a newer version {magenta}({CURRENT_PROJECT_VERSION}){blue} \
            than the latest release {green}({latest_stable_version}).",
            true
        );

        return;
    }

    if CURRENT_PROJECT_VERSION != latest_stable_version {
        output!(
            "{green}\nUpdate available: {CURRENT_PROJECT_VERSION} -> {latest_stable_version}\n",
            true
        );

        update();

        return;
    }

    output!("Already up to date.", true);
}
