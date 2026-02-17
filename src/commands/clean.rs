use crate::utils::{
    colors::COLORS,
    fs::{PATHS, delete_file, delete_folder},
    io::{ask_for_confirmation, input},
    sys::{VARS, args_forced},
};
pub fn clean() {
    let (green, red, reset) = (COLORS.green, COLORS.red, COLORS.reset);

    ask_for_confirmation("Do you want to clean cmdcreate?");

    if !args_forced()
        && input(&format!(
            "\nDo you want to delete old log files?{reset}\n({green}Y{reset} or {red}N{reset})"
        ))
        .trim()
        .eq_ignore_ascii_case("y")
    {
        delete_folder(&format!("{}/.local/share/cmdcreate/logs/", VARS.home));

        println!("{green}\nLog files cleared.{reset}");
    }

    if !args_forced()
        && input(&format!(
            "\nDo you want to delete offline files?{reset}\n({green}Y{reset} or {red}N{reset})"
        ))
        .trim()
        .eq_ignore_ascii_case("y")
    {
        delete_file(&PATHS.changelog);
        delete_file(&PATHS.license);

        println!("{green}\nOffline files cleared.{reset}");
    }

    println!("{green}\nCleaned up cmdcreate.{reset}");
}
