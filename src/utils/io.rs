use std::{io::stdin, process::exit};

use crate::{logger::log, utils::colors::COLORS, utils::sys::args_forced};

pub fn ask_for_confirmation(q: &str) {
    let (red, green, reset) = (COLORS.red, COLORS.green, COLORS.reset);

    log(
        &format!("utils/io::ask_for_confirmation(): Asking: \"{q}\""),
        0,
    );

    if !args_forced()
        && input(&format!("{q}{reset}\n({green}Y{reset} or {red}N{reset})"))
            .trim()
            .to_lowercase()
            != "y"
    {
        log("utils/io::ask_for_confirmation(): Confirmation aborted.", 1);

        println!("{red}\nAborted.{reset}");

        exit(0)
    }
}

pub fn input(text: &str) -> String {
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    log(&format!("utils/io::input(): Input text: \"{text}\""), 0);

    println!("{blue}{text}{reset}");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    return input.trim().to_owned()
}

pub fn error(msg: &str, err: &str) {
    let (red, reset) = (COLORS.red, COLORS.reset);

    log(
        &format!("utils/io::error(): Error: \"{msg}\", Details: \"{err}\""),
        0,
    );

    eprintln!("{red}Error: {} {err}{reset}", msg.trim());

    exit(1)
}

#[derive(Debug)]
pub struct TestError(pub String);

impl core::fmt::Display for TestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::error::Error for TestError {}

pub fn _error_result<T>(msg: &str) -> Result<T, TestError> {
    return Err(TestError(msg.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_returns_err() {
        let result: Result<(), _> = _error_result("nope");
        assert!(result.is_err());
    }

    #[test]
    fn error_message_matches() {
        assert_eq!(_error_result::<()>("bad").unwrap_err().to_string(), "bad");
    }
}
