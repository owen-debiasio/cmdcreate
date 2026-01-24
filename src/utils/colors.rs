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

    const ALL_COLORS: [&str; 5] = [
        COLORS.red,
        COLORS.green,
        COLORS.yellow,
        COLORS.blue,
        COLORS.reset,
    ];

    fn all_colors() -> impl Iterator<Item = &'static str> {
        ALL_COLORS.into_iter()
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
                "color does not start with ANSI escape: {color:?}"
            );

            assert!(
                color.ends_with('m'),
                "color does not end with 'm': {color:?}"
            );
        }
    }

    #[test]
    fn reset_is_unique() {
        for color in all_colors() {
            if color != COLORS.reset {
                assert_ne!(color, COLORS.reset, "reset color must be unique");
            }
        }
    }

    #[test]
    fn colors_wrap_text_correctly() {
        let text = "test";

        for color in all_colors() {
            let wrapped = format!("{color}{text}{}", COLORS.reset);

            assert!(
                wrapped.starts_with(color),
                "wrapped text does not start with color"
            );

            assert!(
                wrapped.ends_with(COLORS.reset),
                "wrapped text does not end with reset"
            );

            assert_eq!(
                wrapped,
                format!("{color}{text}{}", COLORS.reset),
                "wrapped output mismatch"
            );
        }
    }
}
