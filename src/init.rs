use crate::{
    VERSION,
    commands::update::get_install_path,
    configs::init_configs,
    logger::log,
    utils::{
        fs::init_fs_layout,
        sys::{ARCH, VARS, get_distro_base, installation_method},
    },
    version::is_development_version,
};

pub fn debug_intro() -> String {
    format!(
        "                               ----------------
        Welcome to cmdcreate!           Version: {VERSION} {}
            Created by:                 CPU Architecture: {ARCH}
           Owen Debiasio                Distro Base: {:?}
       owen.debiasio@gmail.com          Preferred installation method: {:?}
                                        Preferred text editor: {}
    Have an issue? Copy this text       Home directory: {}
          and open an issue             Shell in use: {}
                                        ----------------",
        if is_development_version() {
            "(devel)"
        } else {
            "(stable)"
        },
        get_distro_base(),
        installation_method(Option::from(get_install_path())),
        VARS.editor,
        VARS.home,
        VARS.shell,
    )
}

pub fn init() {
    init_fs_layout();
    init_configs();

    log(
        &format!(
            "init::init(): Starting cmdcreate...\n         {}",
            debug_intro()
        ),
        0,
    );
}
