fn main() {
    let log_level = if std::env::var("PROFILE").unwrap() == "dev" {
        "debug"
    } else {
        "info"
    };
    println!("cargo:rustc-env=RUST_LOG={}", log_level);
    println!("cargo::rerun-if-changed=build.rs");
}
