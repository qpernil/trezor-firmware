use xbuild::{CLibrary, Result, bail_unsupported};

fn link_virtual_trezor_library(variable: &str, name: &str) {
    use std::path::Path;

    println!("cargo:rerun-if-env-changed={variable}");
    let library = std::env::var(variable).unwrap_or_else(|_| panic!("missing {variable}"));
    let directory = Path::new(&library)
        .parent()
        .unwrap_or_else(|| panic!("{variable} has no parent directory"));
    println!("cargo:rustc-link-search=native={}", directory.display());
    println!("cargo:rustc-link-lib=dylib={name}");
}

fn main() -> Result<()> {
    xbuild::build_and_link("firmware", |lib| {
        lib.import_lib("io")?;
        lib.import_lib("upymod")?;

        lib.add_include("../../rust"); // Cyclic dependency

        if cfg!(feature = "app_loading") {
            lib.add_source("../../api/trezor_api_v1_impl.c");
        }

        if cfg!(feature = "force_bootloader_upgrade") {
            lib.add_define("FORCE_BOOTLOADER_UPGRADE", Some("1"));
        }

        if cfg!(feature = "emulator") {
            lib.add_sources(["src/unix/main.c", "src/unix/main_main.c"]);
        } else if cfg!(feature = "mcu_stm32") {
            lib.add_sources([
                "src/stm32/main.c",
                "src/stm32/header.S",
                "src/stm32/boot_image_embdata.c",
                "src/stm32/coreapp_header.S",
            ]);

            lib.embed_binary(
                xbuild::vendor_header_path("../../models", "firmware")?,
                "vendorheader",
            )?;

            embed_bootloader_binary(lib)?;
            embed_kernel_binary(lib)?;

            if cfg!(feature = "nrf") {
                embed_nrf_app_binary(lib)?;
            }
        } else {
            bail_unsupported!()
        }

        if std::env::var_os("VIRTUAL_TREZOR_HEADLESS_DISPLAY").is_some() {
            lib.add_define("VIRTUAL_TREZOR_HEADLESS_DISPLAY", Some("1"));
        }

        Ok(())
    })?;

    if std::env::var_os("VIRTUAL_TREZOR_HEADLESS_DISPLAY").is_some() {
        link_virtual_trezor_library("VIRTUAL_TREZOR_DISPLAY_LIBRARY", "display_backends");
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }
    if std::env::var_os("VIRTUAL_TREZOR_SUPERVISOR_USB").is_some() {
        link_virtual_trezor_library("VIRTUAL_TREZOR_WORKER_LIBRARY", "usb_gadget_worker");
    }
    Ok(())
}

fn embed_kernel_binary(lib: &mut CLibrary) -> Result<()> {
    let kernel = xbuild::cargo_profile_dir()?.join("kernel.bin");
    lib.embed_binary(&kernel, "kernel")
}

fn embed_bootloader_binary(lib: &mut CLibrary) -> Result<()> {
    let model_id = xbuild::current_model_id()?;
    let model_dir = format!("../../models/{}", model_id);
    let suffix = if cfg!(feature = "bootloader_devel") {
        "_devel"
    } else {
        ""
    };

    let bootloader = format!("{model_dir}/bootloaders/bootloader_{model_id}{suffix}.bin");

    if cfg!(feature = "boot_ucb") {
        // embed uncompressed bootloader image
        lib.embed_binary(bootloader, "bootloader")?;
    } else {
        lib.embed_compressed_binary(bootloader, "bootloader")?;
    }

    Ok(())
}

fn embed_nrf_app_binary(lib: &mut CLibrary) -> Result<()> {
    let model_id = xbuild::current_model_id()?;
    let model_dir = format!("../../models/{}", model_id);
    let suffix = if cfg!(feature = "bootloader_devel") {
        "-dev"
    } else {
        ""
    };
    let nrf_app = format!("{model_dir}/trezor-ble{suffix}.bin");
    lib.embed_binary(&nrf_app, "nrf_app")
}
