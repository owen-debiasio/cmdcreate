use crate::core::configs::load::load_configuration;
use crate::utils::colors::COLORS;
use crate::utils::sys::arguments::args_contains;

pub fn is_verbose_enabled() -> bool {
    let verbose_args_passed = args_contains("-V") || args_contains("--verbose");

    let verbose_enabled_via_config = load_configuration("logs", "verbose", "false")
        .parse::<bool>()
        .unwrap_or(false);

    verbose_args_passed || verbose_enabled_via_config
}

pub fn output_verbose_message(text_to_print: &str) {
    if is_verbose_enabled() {
        // Using print! because your text_to_log already ends with \n
        print!("{text_to_print}{}", COLORS.reset);
    }
}
