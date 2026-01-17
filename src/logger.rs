use std::fs::create_dir_all;
use std::path::Path;

use chrono::Local;

use crate::{
    configs::load,
    utils::{
        colors::COLORS,
        fs::{PATHS, write_to_file},
        sys::args_contains,
    },
};

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

    if args_contains("-V")
        || args_contains("--verbose")
        || load("logs", "verbose", "").parse::<bool>().unwrap_or(false)
    {
        println!("{color}{time} {log_text}{}", COLORS.reset);
    }

    if !Path::new(&PATHS.log_dir).exists()
        && let Err(e) = create_dir_all(&PATHS.log_dir)
    {
        eprintln!("Failed to create log directory: {e}");
        return;
    }

    write_to_file(
        &format!("{}/{time}.log", PATHS.log_dir),
        &format!("{time} {log_text}\n"),
    );
}
