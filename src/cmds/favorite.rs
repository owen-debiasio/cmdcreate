use crate::{
    cmds::tools::is_command_installed,
    utils::{
        colors::COLORS,
        fs::{PATHS, read_file_to_string, remove_from_file, write_to_file},
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
    let favorites_path = &PATHS.favorites;

    is_command_installed(cmd);

    if read_file_to_string(favorites_path)
        .lines()
        .any(|c| c == cmd)
    {
        println!("{yellow}Command {blue}\"{cmd}\"{yellow} is already in favorites.{reset}");
        return;
    }
    write_to_file(favorites_path, &format!("{cmd}\n"));
    println!("{green}Command {blue}\"{cmd}\"{green} added to favorites.{reset}");
}

fn remove(cmd: &str) {
    let (blue, green, reset) = (COLORS.blue, COLORS.green, COLORS.reset);

    remove_from_file(&PATHS.favorites, cmd);
    println!("{green}Command {blue}\"{cmd}\"{green} removed from favorites.{reset}");
}
