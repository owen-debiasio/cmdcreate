pub struct Colors {
    pub reset: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
}

pub const COLORS: Colors = Colors {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m",
};

#[cfg(test)]
mod tests {
    use super::*;

    fn all_colors() -> Vec<&'static str> {
        return vec![
            COLORS.red,
            COLORS.green,
            COLORS.yellow,
            COLORS.blue,
            COLORS.reset,
        ];
    }

    #[test]
    fn all_colors_are_non_empty() {
        for color in all_colors() {
            assert!(!color.is_empty(), "color code should not be empty");
        }
    }

    #[test]
    fn colors_are_valid_ansi_codes() {
        for color in all_colors() {
            assert!(
                color.starts_with("\x1b["),
                "color does not start with ANSI escape"
            );

            assert!(color.ends_with('m'), "color does not end with 'm'");
        }
    }

    #[test]
    fn colors_wrap_text_correctly() {
        let text = "test";

        for color in all_colors() {
            let wrapped = format!("{color}{text}{}", COLORS.reset);

            assert!(wrapped.contains(text));
            assert!(wrapped.starts_with(color));
            assert!(wrapped.ends_with(COLORS.reset));
        }
    }
}
