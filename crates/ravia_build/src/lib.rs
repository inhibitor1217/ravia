use std::{fs, path::Path};

use fs_extra::dir::CopyOptions;

/// Build result type.
pub type Result<T> = anyhow::Result<T>;

/// Build error type.
pub type Error = anyhow::Error;

/// Build the project.
pub fn build() -> Result<()> {
    copy_resources()?;
    set_log_level()?;

    Ok(())
}

/// Copy resources from the engine and user directories to the output directory.
fn copy_resources() -> Result<()> {
    println!("cargo::rerun-if-env-changed=CARGO_MANIFEST_DIR");
    println!("cargo::rerun-if-env-changed=PROFILE");

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH")?;
    let working_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let default_engine_res_dir = Path::new(&working_dir).join("../../crates/ravia_res");
    let default_user_res_dir = Path::new(&working_dir).join("res");

    println!(
        "cargo::rerun-if-changed={}",
        default_engine_res_dir.to_string_lossy()
    );
    println!(
        "cargo::rerun-if-changed={}",
        default_user_res_dir.to_string_lossy()
    );

    let out_dir = if target_arch == "wasm32" {
        String::from(Path::new(&working_dir).join("pkg/static").to_string_lossy())
    } else {
        std::env::var("OUT_DIR")?
    };

    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.content_only = true;

    let assets_out_dir = Path::new(&out_dir).join("res");
    let engine_assets_out_dir = assets_out_dir.join("engine");
    let user_assets_out_dir = assets_out_dir.join("user");

    // Create destination directories
    fs::create_dir_all(engine_assets_out_dir.clone())?;
    fs::create_dir_all(user_assets_out_dir.clone())?;

    // Copy engine resources
    if let Ok(true) = default_engine_res_dir.try_exists() {
        fs_extra::dir::copy(
            default_engine_res_dir.clone(),
            engine_assets_out_dir.clone(),
            &copy_options,
        )?;
    }

    if let Ok(true) = default_user_res_dir.try_exists() {
        fs_extra::dir::copy(
            default_user_res_dir.clone(),
            user_assets_out_dir.clone(),
            &copy_options,
        )?;
    }

    println!(
        "cargo:rustc-env=RAVIA_RES={}",
        assets_out_dir.to_string_lossy()
    );

    Ok(())
}

/// Set the log level based on the build profile.
fn set_log_level() -> Result<()> {
    println!("cargo::rerun-if-env-changed=PROFILE");

    let log_level = if std::env::var("PROFILE").unwrap() == "debug" {
        "debug"
    } else {
        "info"
    };
    println!("cargo:rustc-env=RUST_LOG={}", log_level);

    Ok(())
}
