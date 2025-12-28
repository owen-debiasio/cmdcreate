use crate::{
    cmds::tools::get_installed_commands,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string},
    },
};

pub fn list() {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let installed_scripts = get_installed_commands();
    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_scripts.len()
    );

    for script in installed_scripts {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();
        let favorites = read_file_to_string(&PATHS.favorites);

        if favorites.contains(name.to_string().as_str()) {
            println!("★ {name}");
            continue;
        }

        if favorites.is_empty() {
            println!("{name}");
            continue;
        }

        println!("  {name}");
    }
}
