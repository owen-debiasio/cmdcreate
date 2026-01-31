use crate::{
    VERSION,
    commands::update::get_install_path,
    configs::{init_configs, load},
    logger::log,
    utils::{
        colors::COLORS,
        fs::{init_fs_layout, write_to_file},
        io::input,
        sys::{ARCH, VARS, get_distro_base, installation_method},
    },
};

pub fn debug_intro() -> String {
    format!(
        "                               ----------------
        Welcome to cmdcreate!           Version: {VERSION}
            Created by:                 CPU Architecture: {ARCH}
           Owen Debiasio                Distro Base: {:?}
       owen.debiasio@gmail.com          Preferred installation method: {:?}
                                        Preferred text editor: {}
    Have an issue? Copy this text       Home directory: {}
          and open an issue             Shell in use: {}
                                        ----------------",
        get_distro_base(),
        installation_method(Option::from(get_install_path())),
        VARS.editor,
        VARS.home,
        VARS.shell
    )
}

pub fn init() {
    init_fs_layout();
    init_configs();

    log(
        &format!(
            "init::init(): Starting cmdcreate...\n         {}",
            &debug_intro()
        ),
        0,
    );

    log("init::init(): Checking CPU architecture...", 0);

    if ARCH == "x86_64"
        || load("sys", "spoof_arch", "")
            .parse::<bool>()
            .unwrap_or(false)
    {
        return;
    }

    let (yellow, red, reset) = (COLORS.yellow, COLORS.red, COLORS.reset);

    log(
        "init::init(): Unsupported CPU architecture detected... Displaying warning...",
        1,
    );

    eprintln!(
        "{yellow}Your current CPU architecture {red}({ARCH}){yellow} is not currently supported.
        Use {red}x86_64{yellow} as it is supported.

        (You can disable this message in the configuration file){reset}

        Do you want to disable this message?\n{red}(Y/N){reset}"
    );

    log(
        "init::init(): Asking for confirmation to disable the warning...",
        0,
    );

    if input("").to_lowercase() == "y" {
        write_to_file(
            &format!("{}/.config/cmdcreate/config.toml", VARS.home),
            "[sys]",
            true,
        );

        write_to_file(
            &format!("{}/.config/cmdcreate/config.toml", VARS.home),
            "\nspoof_arch = \"true\"",
            true,
        );
    }
}
