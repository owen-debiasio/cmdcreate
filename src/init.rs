use std::io::stdin;

use crate::{
    configs::{init_configs, load},
    utils::{colors::COLORS, fs::init_fs, sys::ARCH},
};

pub fn init_() {
    let (yellow, red, reset) = (COLORS.yellow, COLORS.red, COLORS.reset);

    init_configs();
    init_fs();

    if ARCH != "x86_64" && !load("sys", "fake_arch", "").parse().unwrap_or(false) {
        for line in [
            format!(
                "{yellow}Your current CPU architecture {red}({ARCH}){yellow} is not currently supported."
            ),
            format!("Use {red}x86_64{yellow} as for it is supported.\n"),
            format!("Press {red}Enter{yellow} to continue...{reset}"),
        ] {
            eprintln!("{line}");
        }

        let mut read = String::new();
        stdin().read_line(&mut read).unwrap();
    }
}
