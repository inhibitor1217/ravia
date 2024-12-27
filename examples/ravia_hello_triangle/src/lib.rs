use ravia_engine::prelude::*;

fn init_log() {
    #[cfg(target_arch = "wasm32")]
    {
        use log;

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let profile = std::env::var("PROFILE").unwrap_or("dev".to_string());
        let log_level = if profile == "dev" {
            log::Level::Debug
        } else {
            log::Level::Info
        };
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

    boot(EngineConfig {
        window_title: "Hello Triangle",
        ..Default::default()
    });
}
