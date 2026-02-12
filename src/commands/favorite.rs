use crate::{
    commands::tools::command_is_installed,
    logger::log,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string, remove_from_file, write_to_file},
        io::error,
    },
};

pub fn favorite(mode: &str, command: &str) {
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    match mode {
        "add" => add(command),
        "remove" => remove(command),

        _ => println!("Usage:\ncmdcreate {blue}favorite {yellow}<add/remove> <command>{reset}"),
    }
}

fn add(cmd: &str) {
    let (blue, green, yellow, reset) = (COLORS.blue, COLORS.green, COLORS.yellow, COLORS.reset);

    log(
        &format!("commands/favorite::add(): Adding command \"{cmd}\" to favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    command_is_installed(cmd);

    if read_file_to_string(favorites_path)
        .lines()
        .any(|c| c == cmd)
    {
        println!("{yellow}Command {blue}\"{cmd}\"{yellow} is already in favorites.{reset}");

        return;
    }

    write_to_file(favorites_path, &format!("{cmd}\n"), true);

    println!("{green}Command {blue}\"{cmd}\"{green} added to favorites.{reset}");
}

fn remove(cmd: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    log(
        &format!("commands/favorite::remove(): Removing command \"{cmd}\" from favorites..."),
        0,
    );

    let favorites_path = &PATHS.favorites;

    if !read_file_to_string(favorites_path)
        .lines()
        .any(|c| c == cmd)
    {
        error("Command isn't in favorites:", cmd);
    }

    remove_from_file(favorites_path, cmd);

    println!("{green}Command {blue}\"{cmd}\"{green} removed from favorites.{reset}");
}
