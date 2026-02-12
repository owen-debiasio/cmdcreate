use crate::{
    commands::{create::create, favorite::favorite as add_favorite},
    logger::log,
    utils::{colors::COLORS, fs::read_file_to_string, io::error},
};

pub fn import(file: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/import::import(): Importing commands from file: \"{file}\"..."),
        0,
    );

    let contents = read_file_to_string(file);

    if contents.trim().is_empty() {
        error("Import file is empty or unreadable.", "");
    }

    for line in contents.replace("[|", "|").lines() {
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        if line.trim().is_empty() && !parts.is_empty() {
            continue;
        }

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

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        create(name, &data, false);

        if favorite {
            add_favorite("add", name);
        }
    }

    println!("\n{green}Successfully imported commands.{reset}");
}
