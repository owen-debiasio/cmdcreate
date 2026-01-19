use std::{io::stdin, process::exit};

use crate::{
    logger::log,
    utils::{colors::COLORS, sys::args_forced},
};

pub fn ask_for_confirmation(q: &str) {
    let (red, green, reset) = (COLORS.red, COLORS.green, COLORS.reset);

    log(
        &format!("utils/io::ask_for_confirmation(): Asking: \"{q}\""),
        0,
    );

    if !args_forced()
        && input(&format!("{q}{reset}\n({green}Y{reset} or {red}N{reset})"))
        .trim()
        .to_lowercase()
        != "y"
    {
        log("utils/io::ask_for_confirmation(): Confirmation aborted.", 1);

        println!("{red}\nAborted.{reset}");

        exit(0)
    }
}

pub fn input(text: &str) -> String {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log(&format!("utils/io::input(): Input text: \"{text}\"\n"), 0);

    println!("{blue}{text}{reset}");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    log(
        &format!("utils/io::input(): User input: \"{}\"", input.trim()),
        0,
    );

    input.trim().to_string()
}

pub fn error(msg: &str, err: &str) {
    let (red, reset) = (COLORS.red, COLORS.reset);

    log(
        &format!("utils/io::error(): Error encountered: Message: \"{msg}\", Error: \"{err}\""),
        0,
    );

    eprintln!("{red}Error: {} {err}{reset}", msg.trim());

    exit(1)
}
