use crate::{commands::config::main::AVAILABLE_SETTINGS, output, utils::colors::COLORS};

pub fn list() {
    let cyan = COLORS.cyan;

    output!("Available settings:", true);

    for setting in AVAILABLE_SETTINGS {
        println!();

        let setting_list_splitter = setting.split(':');
        let category = setting_list_splitter.clone().next().unwrap();
        let keys = setting_list_splitter
            .clone()
            .next_back()
            .unwrap()
            .split(',');

        output!("{cyan}{category}");

        for key in keys {
            let reset = COLORS.reset;
            output!("{reset}{key}");
        }
    }
}
