use crate::{
    configs::{init_configs, load},
    logger::log,
    utils::{colors::COLORS, fs::init_fs, io::input, sys::ARCH},
};

pub fn init() {
    init_configs();
    init_fs();

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

        (You can disable this message in the configuration file){reset}"
    );

    let _ = input("");
}
