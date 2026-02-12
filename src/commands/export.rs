use crate::{
    commands::tools::get_installed_commands,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, create_file, read_file_to_string, write_to_file},
    },
};

pub fn export(path: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let export_file = &format!("{path}/export.cmdcreate");

    log(
        &format!("commands/export::export(): Exporting commands to: \"{export_file}\"..."),
        0,
    );

    log("commands/export::export(): Creating export file...", 0);

    create_file(export_file);

    for script in get_installed_commands() {
        if let Some(stem) = script.file_stem() {
            let cmd = stem.to_string_lossy();

            log(
                &format!("commands/export::export(): Exporting command: \"{cmd}\"..."),
                0,
            );

            let cmd_contents =
                read_file_to_string(&format!("{}{cmd}", PATHS.install_dir)).replace('|', "[|");

            let data = if read_file_to_string(&PATHS.favorites).contains(cmd.as_ref()) {
                format!("{cmd} | {cmd_contents} | favorite\n")
            } else {
                format!("{cmd} | {cmd_contents}\n")
            };

            log(
                &format!("commands/export::export(): Writing data to file: \"{data}\"..."),
                0,
            );

            write_to_file(export_file, &data, true);
        }
    }

    log(
        "commands/export::export(): Exporting process completed...",
        0,
    );

    println!("{green}Successfully exported commands to:{blue} \"{export_file}\"{green}.{reset}",);
}
