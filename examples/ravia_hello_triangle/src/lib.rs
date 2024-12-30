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
        init_world,
        ..Default::default()
    });
}

fn init_world(world: &mut World, ctx: &EngineContext) {
    let camera = Camera::noop(ctx);
    world.push((camera,));

    let mesh = Mesh::new::<Vertex2DColor>(
        ctx,
        vec![
            Vertex2DColor {
                position: vec2(-0.5, -0.5),
                data: vec3(1.0, 0.0, 0.0),
            },
            Vertex2DColor {
                position: vec2(0.5, -0.5),
                data: vec3(0.0, 1.0, 0.0),
            },
            Vertex2DColor {
                position: vec2(0.0, 0.5),
                data: vec3(0.0, 0.0, 1.0),
            },
        ],
    );

    let material = Material::new(
        ctx,
        &ShaderConfig::new(include_str!("triangle.wgsl")).with_vertex_type::<Vertex2DColor>(),
    );

    world.push((mesh, material));
}
