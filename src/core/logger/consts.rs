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

use crate::utils::colors::COLORS;

#[derive(Copy, Clone)]
pub enum Severity {
    Normal = 0,
    Warn = 1,
}

impl Severity {
    pub fn to_colored_string(self) -> String {
        let (cyan, yellow, reset) = (COLORS.cyan, COLORS.yellow, COLORS.reset);

        match self {
            Self::Normal => format!("{cyan}LOG{reset}"),
            Self::Warn => format!("{yellow}WARN{reset}"),
        }
    }
}
