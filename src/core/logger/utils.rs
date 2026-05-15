use crate::core::configs::load::load_configuration;
use crate::utils::fs::core::write_to_file;
use crate::utils::fs::paths::PATHS;
use chrono::Local;

pub fn get_formatted_timestamp() -> String {
    let time_format = load_configuration("logs", "time_format", "%Y-%m-%d %H:%M:%S");
    Local::now().format(&time_format).to_string()
}

pub fn save_log_to_file(timestamp: &str, plain_text: &str) {
    let log_dir = &PATHS.log_directory;
    let log_file_name = format!("{log_dir}/{timestamp}.log");
    write_to_file(&log_file_name, plain_text, true);
}
