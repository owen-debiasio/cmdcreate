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
    core::meta::version::consts::CURRENT_PROJECT_VERSION,
    utils::{git::get_latest_tag, net::not_connected_to_internet},
};
use std::cmp::Ordering;

pub fn is_development_version() -> bool {
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
    if is_development_version() {
        if not_connected_to_internet() {
            "(build status unknown)"
        } else {
            "(development)"
        }
    } else {
        "(stable)"
    }
}
