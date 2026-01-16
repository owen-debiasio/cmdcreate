use crate::utils::{
    colors::COLORS,
    fs::{PATHS, read_file_to_string, write_to_file},
    sys::run_shell_command,
};

pub fn import(file: &str) {
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    let contents = read_file_to_string(file);

    if contents.trim().is_empty() {
        println!("{yellow}Import file is empty or unreadable.{reset}");
        return;
    }

    for line in contents.replace("[|", "|").lines() {
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();

        if line.trim().is_empty() && !parts.is_empty() {
            continue;
        }

        let name = parts[0];
        let mut data = String::new();
        let mut favorite = false;

        for part in parts.iter().skip(1) {
            if *part == "favorite" {
                favorite = true;
            } else if !part.is_empty() {
                if data.is_empty() {
                    data.push('\n');
                }

                data.push_str(part);
            }
        }

        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        let script_path = format!("{}{name}", PATHS.install_dir);

        write_to_file(&script_path, &data);

        run_shell_command(&format!(
            "chmod +x {script_path}; sudo ln -sf {script_path} /usr/bin/{name}"
        ));

        if favorite {
            write_to_file(&PATHS.favorites, &format!("{name}\n"));
        }
    }

    println!("\n{green}Successfully imported commands.{reset}");
}
