/// Engine name.
pub const ENGINE_NAME: &str = "ravia_engine";

/// Engine version.
pub const ENGINE_VERSION: &str = "0.1.0";

/// Starts the engine.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn boot() {
    log::info!(target: "ravia_engine", "Booting {} {}", ENGINE_NAME, ENGINE_VERSION);
}
