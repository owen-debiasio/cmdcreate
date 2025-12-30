use crate::{
    cmds::tools::get_installed_commands,
    utils::{
        colors::COLORS,
        fs::{PATHS, create_file, read_file_to_string, write_to_file},
    },
};

pub fn export(path: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let export_file = &format!("{path}/export.cmdcreate");
    create_file(export_file);

    for script in get_installed_commands() {
        if let Some(stem) = script.file_stem() {
            let cmd = stem.to_string_lossy();

            let mut cmd_contents = read_file_to_string(&format!("{}{cmd}", PATHS.install_dir));

            cmd_contents = cmd_contents.replace('|', "[|");

            let line = if read_file_to_string(&PATHS.favorites).contains(cmd.as_ref()) {
                format!("{cmd} | {cmd_contents} | favorite\n")
            } else {
                format!("{cmd} | {cmd_contents}\n")
            };

            write_to_file(export_file, &line);
        }
    }

    println!("{green}Successfully exported commands to:{blue} \"{export_file}\"{green}.{reset}",);
}
