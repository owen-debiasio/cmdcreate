use crate::{
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, overwrite_file},
        io::error,
        sys::run_shell_command,
    },
};

pub fn create(command: &str, contents: &str, verbose: bool) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!(
            "cmds/create::create(): Creating command \"{command}\": With contents \"{contents}\" | (verbose={verbose})"
        ),
        0,
    );

    let script = &format!("{}{command}", PATHS.install_dir);

    log(
        &format!("cmds/create::create(): Script path: \"{script}\""),
        0,
    );

    if contents.is_empty() {
        error("The contents of your command can not be empty.", "");
    }

    log(
        "cmds/create::create(): Command isn't empty, continuing...",
        0,
    );

    log(
        &format!("cmds/create::create(): Writing contents to script: \"{script}\""),
        0,
    );

    overwrite_file(script, contents);

    log("cmds/create::create(): Activating command...", 0);

    run_shell_command(&format!(
        "
        chmod +x {script}; \
        sudo ln -sf {script} /usr/local/bin/{command}
        ",
    ));

    if verbose {
        log(
            "cmds/create::create(): Verbose, printing success message...",
            0,
        );
        println!("\n{green}Success! Created command: {blue}\"{command}\"{reset}");
        return;
    }

    log(
        "cmds/create::create(): Not verbose, not printing success message...",
        0,
    );
}
