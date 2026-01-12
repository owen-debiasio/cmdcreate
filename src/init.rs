use std::io::stdin;

use crate::{
    configs::{init_configs, load},
    utils::{colors::COLORS, fs::init_fs, sys::ARCH},
};

pub fn init() {
    let (yellow, red, reset) = (COLORS.yellow, COLORS.red, COLORS.reset);

    init_configs();
    init_fs();

    if !(ARCH == "x86_64" && load("sys", "spoof_arch", "").parse().unwrap_or(false)) {
        for line in [
            format!(
                "{yellow}Your current CPU architecture {red}({ARCH}){yellow} is not currently supported."
            ),
            format!("Use {red}x86_64{yellow} as for it is supported.\n"),
            format!("(You can disable this message in the configuration file){reset}\n"),
        ] {
            eprintln!("{line}");
        }

        stdin().read_line(&mut String::new()).unwrap();
    }
}
