use std::io::{self, Read};

use crate::{
    configs::{init_configs, load},
    utils::{colors::COLORS, fs::init_fs, sys::ARCH},
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

    let _ = io::stdin().read(&mut [0u8]).ok();
}
