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
