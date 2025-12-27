use crate::utils::fs::{PATHS, path_exists, read_file_to_string};
use crate::{
    cmds::{create::create, tools::get_installed_commands},
    utils::colors::COLORS,
};

pub fn repair() {
    let (green, yellow, blue, reset) = (COLORS.green, COLORS.yellow, COLORS.blue, COLORS.reset);

    let mut count = 0;
    for script in get_installed_commands() {
        let name = script.file_stem().unwrap_or_default().to_string_lossy();

        if !path_exists(&format!("/usr/bin/{name}")) {
            println!("{green}Repairing command: {blue}\"{name}\"{reset}");

            create(
                &name,
                &read_file_to_string(&format!("{}{name}", PATHS.install_dir)),
                false,
            );

            count += 1;
        }
    }

    if count > 0 {
        println!("{green}Broken commands have been repaired.{reset}");
        return;
    }
    
    println!("{yellow}No commands needed repairs.{reset}");
}
