/// Module for terminal color formatting
///
/// This module provides ANSI escape codes for colored output in terminal
/// environments. It includes a set of commonly used colors and a reset code
/// to return to default terminal colors.
/// Structure containing ANSI escape codes for terminal colors
///
/// Each field represents a different color or formatting option:
/// - `reset`: Returns terminal color to default
/// - `red`: Used for errors and warnings
/// - `green`: Used for success messages
/// - `yellow`: Used for important values and highlights
/// - `blue`: Used for commands and actions
/// - `magenta`: Used for flags and options
/// - `cyan`: Used for auxiliary information
pub struct Colors {
    pub reset: &'static str,
    pub red: &'static str,
    pub green: &'static str,
    pub yellow: &'static str,
    pub blue: &'static str,
    pub magenta: &'static str,
    pub cyan: &'static str,
}

/// Global constant containing all available terminal colors
///
/// Usage example:
/// ```rust
/// println!("{}Success!{}", COLORS.green, COLORS.reset);
/// println!("{}Error:{} {}", COLORS.red, COLORS.reset, "Something went wrong");
/// ```
pub const COLORS: Colors = Colors {
    reset: "\x1b[0m",
    red: "\x1b[31m",
    green: "\x1b[32m",
    yellow: "\x1b[33m",
    blue: "\x1b[34m",
    magenta: "\x1b[35m",
    cyan: "\x1b[36m",
};
