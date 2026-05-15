use crate::core::logger::consts::Severity;
use crate::core::logger::utils::{get_formatted_timestamp, save_log_to_file};
use crate::core::logger::verbose::output_verbose_message;
use crate::utils::colors::{COLORS, remove_spare_color_codes};

pub fn log(text_to_log: &str, importance_level: Severity) {
    let time = get_formatted_timestamp();

    let log_type = importance_level.to_colored_string();

    let (blue, reset) = (COLORS.blue, COLORS.reset);

    let console_output = format!("[{blue}{time}{reset}] [{log_type}] {text_to_log}\n");
    output_verbose_message(&console_output);

    let file_output = remove_spare_color_codes(console_output);
    save_log_to_file(&time, &file_output);
}
