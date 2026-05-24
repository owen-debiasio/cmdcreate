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
    core::meta::{
        project::{
            author_information::AUTHOR, get_project_copyright_info, project_information::PROJECT,
        },
        version::{build::get_build_status, consts::CURRENT_PROJECT_VERSION},
    },
    output,
};

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
