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
        window_title: "Hello Texture",
        init_world,
        gpu: GpuConfig {
            default_shader: ShaderConfig::new(include_str!("triangle_tex.wgsl"))
                .with_vertex_type::<Vertex2DTexture>()
                .with_bound_texture::<Texture2D>(),
        },
        ..Default::default()
    });
}

fn init_world(world: &mut World) {
    world.push((Mesh::<Vertex2DTexture>::new(
        vec![
            Vertex2DTexture {
                position: [-0.5, -0.5],
                data: [0.0, 1.0],
            },
            Vertex2DTexture {
                position: [0.5, -0.5],
                data: [1.0, 1.0],
            },
            Vertex2DTexture {
                position: [-0.5, 0.5],
                data: [0.0, 0.0],
            },
            Vertex2DTexture {
                position: [0.5, 0.5],
                data: [1.0, 0.0],
            },
        ],
        vec![0, 1, 3, 0, 3, 2],
    ),));
}
