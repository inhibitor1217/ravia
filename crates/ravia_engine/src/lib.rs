pub mod engine;
pub mod graphics;

/// Engine name.
pub const ENGINE_NAME: &str = "ravia_engine";

/// Engine version.
pub const ENGINE_VERSION: &str = "0.1.0";

/// Starts the engine.
pub fn boot(config: engine::EngineConfig) {
    log::info!(target: "ravia_engine", "Booting {} {}", ENGINE_NAME, ENGINE_VERSION);

    engine::Engine::run(config);
}

pub mod prelude {
    pub use crate::engine::*;
    pub use crate::graphics::*;

    pub use crate::boot;
    pub use crate::ENGINE_NAME;
    pub use crate::ENGINE_VERSION;

    #[cfg(target_arch = "wasm32")]
    pub use wasm_bindgen::prelude::*;
}
