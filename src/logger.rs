use crate::{
    PATHS,
    configs::load,
    utils::{colors::COLORS, fs::write_to_file, sys::args_contains},
};
use chrono::Local;

pub fn log(text: &str, lvl: u8) {
    let time = Local::now()
        .format(&load("logs", "time_format", "%Y-%m-%d %H:%M:%S"))
        .to_string();

    let color = match lvl {
        0 => COLORS.cyan,
        1 => COLORS.yellow,
        2 => COLORS.red,
        _ => COLORS.reset,
    };

    let log_text = format!("{}[LOG] {color}{text}{}", COLORS.blue, COLORS.reset);

    if load("logs", "verbose", "").parse::<bool>().unwrap_or(false)
        || args_contains("-V")
        || args_contains("--verbose")
    {
        println!("{color}{log_text}{}", COLORS.reset);
    }

    write_to_file(
        &format!("{}/{time}.log", PATHS.log_dir),
        &format!("{time} {log_text}\n"),
    );
}
