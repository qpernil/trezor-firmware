use xbuild::{CLibrary, Result, bail_unsupported};

pub fn def_module(lib: &mut CLibrary) -> Result<()> {
    lib.add_include("rng/inc");

    if cfg!(feature = "emulator") {
        println!("cargo:rerun-if-env-changed=VIRTUAL_TREZOR_OS_RNG");
        if std::env::var("VIRTUAL_TREZOR_OS_RNG").as_deref() == Ok("1") {
            lib.add_define("VIRTUAL_TREZOR_OS_RNG", Some("1"));
            lib.add_sources(["rng/unix/rng.c", "rng/stm32/rng_use_flags.c"]);
        } else {
            lib.add_define("USE_INSECURE_PRNG", Some("1"));
            lib.add_sources([
                "rng/unix/rng.c",
                "rng/unix/rng_use_flags.c",
                "rng/unix/rng_mock.c",
            ]);
        }
    } else if cfg!(feature = "mcu_stm32") {
        lib.add_sources(["rng/stm32/rng.c", "rng/stm32/rng_use_flags.c"]);
    } else {
        bail_unsupported!();
    }

    Ok(())
}
