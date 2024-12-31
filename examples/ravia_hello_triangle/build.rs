use ravia_build::*;

fn main() -> Result<()> {
    println!("cargo::rerun-if-changed=build.rs");

    build()?;

    Ok(())
}
