use crate::{
    configs::{init_configs, load},
    utils::{colors::COLORS, fs::init_fs, io::input, sys::ARCH},
};

pub fn init() {
    init_configs();
    init_fs();

    if ARCH == "x86_64"
        || load("sys", "spoof_arch", "")
            .parse::<bool>()
            .unwrap_or(false)
    {
        return;
    }

    let (yellow, red, reset) = (COLORS.yellow, COLORS.red, COLORS.reset);

    eprintln!(
        "{yellow}Your current CPU architecture {red}({ARCH}){yellow} is not currently supported.
        Use {red}x86_64{yellow} as it is supported.

        (You can disable this message in the configuration file){reset}"
    );

    let _ = input("");
}
