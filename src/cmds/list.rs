use crate::{
    cmds::tools::get_installed_commands,
    configs::load,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{read_file_to_string, PATHS},
    },
};

pub fn list() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log("cmds/list::list(): Gathering installed commands...", 0);

    let installed_scripts = get_installed_commands();
    let amount = installed_scripts.len();

    log(
        &format!(
            "cmds/list::list(): Listing amount of installed commands ({blue}{amount}{reset})..."
        ),
        0,
    );

    println!("Installed commands: ({blue}{amount}{reset})\n--------");

    log("cmds/list::list(): Listing installed commands...", 0);

    for script in installed_scripts {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();
        let favorites = read_file_to_string(&PATHS.favorites);

        log(
            &format!(
                "cmds/list::list(): Current command: \"{name}\" (favorited={})",
                favorites.contains(name.to_string().as_str())
            ),
            0,
        );

        if favorites.contains(name.to_string().as_str()) {
            println!("{} {name}", load("appearence", "favorite_indicator", "★"));
            continue;
        }

        if favorites.is_empty() {
            println!("{name}");
            continue;
        }

        println!("  {name}");
    }
}
