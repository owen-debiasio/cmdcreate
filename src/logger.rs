use chrono::Local;

use crate::{
    configs::load,
    utils::{
        colors::COLORS,
        fs::{PATHS, create_folder, write_to_file},
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

    let (log_text, log_dir) = (format!("[{log_type}] {text}"), &PATHS.log_dir);

    if args_contains("-V")
        || args_contains("--verbose")
        || load("logs", "verbose", "").parse::<bool>().unwrap_or(false)
    {
        println!("{color}{time} {log_text}{}", COLORS.reset);
    }

    create_folder(log_dir);

    write_to_file(
        &format!("{log_dir}/{time}.log"),
        &format!("{time} {log_text}\n"),
        true,
    );
}
