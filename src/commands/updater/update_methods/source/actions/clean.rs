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
    output,
    utils::{fs::core::delete_folder, io::ask_for_confirmation},
};

pub fn cleanup() {
    output!("Cleaning up...", true);

    ask_for_confirmation("Do you want to remove unneeded dependencies?", false);

    delete_folder("/root/.cargo");
    delete_folder("/root/.rustup");
}
