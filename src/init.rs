use crate::{
    VERSION,
    configs::init_configs,
    logger::log,
    utils::{
        fs::init_fs_layout,
        io::error,
        sys::{ARCH, VARS, get_distro_base, installation_method, is_root},
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
        installation_method(),
        VARS.editor,
        VARS.home,
        VARS.shell,
    )
}

pub fn init() {
    if !is_root() {
        error("Please run cmdcreate as root.", "")
    }

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
