use std::{io::stdin, process::exit};

use crate::utils::{colors::COLORS, sys::args_forced};

pub fn ask_for_confirmation(q: &str) {
    let (red, green, reset) = (COLORS.red, COLORS.green, COLORS.reset);

    if !args_forced()
        && input(&format!("{q}{reset}\n({green}Y{reset} or {red}N{reset})"))
            .trim()
            .to_lowercase()
            != "y"
    {
        println!("{red}\nAborted.{reset}");

        exit(0)
    }
}

pub fn input(text: &str) -> String {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    println!("{blue}{text}{reset}");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn error(msg: &str, err: &str) {
    let (red, reset) = (COLORS.red, COLORS.reset);

    eprintln!("{red}Error: {} {err}{reset}", msg.trim());

    exit(1)
}
