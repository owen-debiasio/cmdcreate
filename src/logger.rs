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

    let (color, log_type) = match lvl {
        0 => (COLORS.cyan, "LOG"),
        1 => (COLORS.yellow, "WARN"),
        2 => (COLORS.red, "ERROR"),
        _ => (COLORS.reset, "LOG"),
    };

    let log_text = format!("[{log_type}] {text}");

    if load("logs", "verbose", "").parse::<bool>().unwrap_or(false)
        || args_contains("-V")
        || args_contains("--verbose")
    {
        println!("{color}{time} {log_text}{}", COLORS.reset);
    }

    write_to_file(
        &format!("{}/{}.log", PATHS.log_dir, time),
        &format!("{time} {log_text}\n"),
    );
}
