#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn init_log() {
    #[cfg(target_arch = "wasm32")]
    {
        use log;

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let log_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
        let log_level = log_level.parse::<log::Level>().unwrap_or(log::Level::Info);
        console_log::init_with_level(log_level).expect("Failed to initialize console logger");
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn run() {
    init_log();

    ravia_engine::boot();
}
