use ravia_engine::prelude::*;

fn init_log() {
    #[cfg(target_arch = "wasm32")]
    {
        use log;

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info)
            .expect("Failed to initialize console logger");
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

#[cfg(test)]
mod tests {
    use ravia_engine::prelude::*;

    #[test]
    fn can_load_shader_config_from_file() {
        let config = ravia_engine::shader_config!("triangle.wgsl", "triangle");

        assert_eq!(config.name, Some("triangle"));
        assert!(config.source.is_some());
        assert_eq!(config.vertex_entry_point, "vs_main");
        assert_eq!(config.fragment_entry_point, "fs_main");
    }
}
