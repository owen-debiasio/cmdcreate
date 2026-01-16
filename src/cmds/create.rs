use crate::utils::{
    colors::COLORS,
    fs::{PATHS, overwrite_file},
    io::error,
    sys::run_shell_command,
};

pub fn create(command: &str, contents: &str, verbose: bool) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    let script = &format!("{}{command}", PATHS.install_dir);

    if contents.is_empty() {
        error("The contents of your command can not be empty.", "");
    }

    overwrite_file(script, contents);

    run_shell_command(&format!(
        "
            chmod +x {script}; \
            sudo ln -sf {script} /usr/bin/{command}
            ",
    ));

    if verbose {
        println!("\n{green}Success! Created command: {blue}\"{command}\"{reset}",);
    }
}
