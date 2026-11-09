use xbuild::{CLibrary, Result, bail_unsupported};

fn virtual_trezor_source(name: &str) -> String {
    println!("cargo:rerun-if-env-changed={name}");
    std::env::var(name).unwrap_or_else(|_| panic!("missing {name}"))
}

pub fn def_module(lib: &mut CLibrary) -> Result<()> {
    lib.add_include("button/inc");

    lib.add_define("USE_BUTTON", Some("1"));

    lib.add_source("button/button_poll.c");

    if cfg!(feature = "usb_iface_debug") {
        lib.add_source("button/button_debug.c");
    }

    if cfg!(feature = "emulator") && std::env::var_os("VIRTUAL_TREZOR_HEADLESS_DISPLAY").is_some() {
        lib.add_include("button");
        lib.add_source(virtual_trezor_source("VIRTUAL_TREZOR_BUTTON_SOURCE"));
        if let Ok(resources) = std::env::var("VIRTUAL_TREZOR_BUTTON_RESOURCES_SOURCE")
            && !resources.is_empty()
        {
            println!("cargo:rerun-if-env-changed=VIRTUAL_TREZOR_BUTTON_RESOURCES_SOURCE");
            lib.add_source(resources);
        }
    } else if cfg!(feature = "emulator") {
        lib.add_source("button/unix/button.c");
    } else if cfg!(feature = "mcu_stm32") {
        lib.add_source("button/stm32/button.c");
    } else {
        bail_unsupported!();
    }

    Ok(())
}
