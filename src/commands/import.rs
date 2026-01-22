use crate::{
    commands::{create::create, favorite::favorite as add_favorite},
    logger::log,
    utils::{colors::COLORS, fs::read_file_to_string, io::error},
};

pub fn import(file: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("cmds/backup/import::import(): Importing commands from file: \"{file}\"..."),
        0,
    );

    let contents = read_file_to_string(file);

    if contents.trim().is_empty() {
        error("Import file is empty or unreadable.", "");

        return;
    }

    log(
        "cmds/backup/import::import(): Initializing import process...",
        0,
    );

    for line in contents.replace("[|", "|").lines() {
        log("cmds/backup/import::import(): Splitting lines...", 0);

        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        if line.trim().is_empty() && !parts.is_empty() {
            log(
                "cmds/backup/import::import(): Line is empty, skipping...",
                1,
            );

            continue;
        }

        log(
            "cmds/backup/import::import(): Gathering command parts...",
            0,
        );

        let name = parts.first().unwrap();
        let mut data = String::new();
        let mut favorite = false;

        for part in parts.iter().skip(1) {
            if *part == "favorite" {
                favorite = true;
            } else {
                if data.is_empty() {
                    data.push('\n');
                }

                data.push_str(part);
            }
        }

        log(
            "cmds/backup/import::import(): Initializing import process...",
            0,
        );

        log(
            &format!("cmds/backup/import::import(): Installing command \"{name}\"..."),
            0,
        );

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        log(
            &format!("cmds/backup/import::import(): Creating command \"{name}\"..."),
            0,
        );

        create(name, &data, false);

        if favorite {
            log(
                &format!("cmds/backup/import::import(): Adding command \"{name}\" to favorites..."),
                0,
            );

            add_favorite("add", name);
        }
    }

    log(
        "cmds/backup/import::import(): Import process completed...",
        0,
    );

    println!("\n{green}Successfully imported commands.{reset}");
}
