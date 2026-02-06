mod commands;
mod configs;
mod init;
mod logger;
mod parse;
mod usage;
mod utils;
mod version;

use crate::{
    init::init,
    logger::log,
    parse::parse,
    usage::cmdcreate_usage,
    utils::{fs::init_git_fs, sys::return_args},
    version::VERSION,
};

fn main() {
    init();

    log("main::main(): Retrieving args...", 0);

    let mut args = return_args();
    if args.is_empty() {
        log("main::main(): Args are empty, displaying usage...", 1);

        cmdcreate_usage();

        return;
    }

    if args
        .iter()
        .any(|a| matches!(a.as_str(), "-V" | "--verbose"))
    {
        args.retain(|a| !matches!(a.as_str(), "-V" | "--verbose"));
    }

    cmdcreate(&args);
}

fn cmdcreate(args: &[String]) {
    let mut i = 0;

    while let Some(cmd) = args.get(i).map(String::as_str) {
        if !cmd.starts_with('-') {
            break;
        }

        if matches!(cmd, "-V" | "--verbose") {
            i += 1;
            continue;
        }

        if matches!(
            cmd,
            "-l" | "--license" | "-c" | "--changelog" | "-g" | "--get_offline_files"
        ) {
            log("main::main(): Offline files have been requested...", 0);
            init_git_fs();
        }

        log(&format!("main::main(): Processing command \"{cmd}\"..."), 0);
        parse(cmd, args);

        i += 1;
    }

    if let Some(cmd) = args.get(i).map(String::as_str) {
        log(&format!("main::main(): Processing command \"{cmd}\"..."), 0);
        parse(cmd, &args[i..]);
    }
}
