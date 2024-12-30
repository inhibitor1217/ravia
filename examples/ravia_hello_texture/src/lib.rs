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
        ..Default::default()
    });
}

fn init_world(world: &mut World, ctx: &EngineContext) {
    let camera = Camera::noop(ctx);
    world.push((camera, Transform::identity(ctx, true)));

    let mesh = Mesh::new_indexed::<Vertex2DTexture>(
        ctx,
        vec![
            Vertex2DTexture {
                position: vec2(-0.5, -0.5),
                data: vec2(0.0, 1.0),
            },
            Vertex2DTexture {
                position: vec2(0.5, -0.5),
                data: vec2(1.0, 1.0),
            },
            Vertex2DTexture {
                position: vec2(-0.5, 0.5),
                data: vec2(0.0, 0.0),
            },
            Vertex2DTexture {
                position: vec2(0.5, 0.5),
                data: vec2(1.0, 0.0),
            },
        ],
        vec![0, 1, 3, 0, 3, 2],
    );

    let mut material = Material::new(
        ctx,
        &ShaderConfig::new(include_str!("triangle_tex.wgsl"))
            .with_vertex_type::<Vertex2DTexture>()
            .with_uniforms(&[UniformType::Texture2D]),
    );
    let texture = Texture::default_2d(ctx);
    material.texture = Some(texture);

    world.push((mesh, material, Transform::identity(ctx, false)));
}
