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

use std::env::consts::ARCH as ARCHITECTURE;

use crate::{
    core::logger::{consts::Severity, main::log},
    utils::io::error,
};

pub static ARCH: &str = ARCHITECTURE;

/// Provides support for the following:
/// 32-bit (`x86`, `i386`, `i686`, `armv7`)
/// 64-bit (`x86_64`, `aarch64`)
pub fn arch_is_supported() -> bool {
    ARCH == "x86_64"
        || ARCH == "x86"
        || ARCH == "i386"
        || ARCH == "i686"
        || ARCH == "aarch64"
        || ARCH == "armv7"
}

pub fn cpu_arch_check(error_reason: &str) {
    log(
        "utils/sys::cpu_arch_check(): Double checking if CPU arch. is supported...",
        Severity::Normal,
    );

    if !arch_is_supported() {
        error(error_reason, None)
    }

    log(
        "utils/sys::cpu_arch_check(): CPU arch. is supported... Continuing action...",
        Severity::Normal,
    );
}
