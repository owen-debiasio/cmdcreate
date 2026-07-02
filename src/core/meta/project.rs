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

use crate::core::meta::project::author_information::AUTHOR;

pub const YEAR: &str = "2026";

pub mod author_information {
    pub struct Author {
        pub name: &'static str,
        pub username: &'static str,
        pub email: &'static str,
    }

    pub const AUTHOR: Author = Author {
        name: "Owen Debiasio",
        username: "owen-debiasio",
        email: "owen.debiasio@gmail.com",
    };
}

pub mod project_information {
    pub struct Project {
        pub name: &'static str,
        pub repository: &'static str,
        pub repository_raw: &'static str,
        pub repository_api: &'static str,
    }

    pub const PROJECT: Project = Project {
        name: "cmdcreate",
        repository: "https://github.com/owen-debiasio/cmdcreate",
        repository_raw: "https://raw.githubusercontent.com/owen-debiasio/cmdcreate/main/",
        repository_api: "https://api.github.com/repos/owen-debiasio/cmdcreate/",
    };
}

pub fn get_project_copyright_info() -> String {
    let project_author = AUTHOR.name;
    let author_email = AUTHOR.email;

    format!("Copyright {YEAR} {project_author} <{author_email}>")
}
