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
        window_title: "Hello 3D",
        init_world,
        init_system,
        ..Default::default()
    });
}

#[derive(Debug)]
struct SampleMovement {}

fn init_world(world: &mut World, ctx: &EngineContext) {
    let camera = Camera::perspective_with_defaults(ctx);
    world.push((camera, Transform::identity(ctx)));

    let mesh = Mesh::new_indexed::<Vertex3DTexture>(
        ctx,
        vec![
            Vertex3DTexture {
                position: vec3(-0.5, -0.5, 1.0),
                data: vec2(0.0, 1.0),
            },
            Vertex3DTexture {
                position: vec3(0.5, -0.5, 1.0),
                data: vec2(1.0, 1.0),
            },
            Vertex3DTexture {
                position: vec3(-0.5, 0.5, 1.0),
                data: vec2(0.0, 0.0),
            },
            Vertex3DTexture {
                position: vec3(0.5, 0.5, 1.0),
                data: vec2(1.0, 0.0),
            },
        ],
        vec![0, 1, 3, 0, 3, 2],
    );

    let mut material = Material::new(
        ctx,
        &ShaderConfig::new(include_str!("triangle_tex_perspective.wgsl"))
            .with_vertex_type::<Vertex3DTexture>()
            .with_uniforms(&[
                UniformType::Texture2D,
                UniformType::Camera,
                UniformType::CameraTransform,
                UniformType::ModelTransform,
            ]),
    );
    let texture = Texture::default_2d(ctx);
    material.texture = Some(texture);

    world.push((mesh, material, Transform::identity(ctx), SampleMovement {}));
}

fn init_system(builder: &mut systems::Builder) {
    builder.add_system(sample_movement_system());
}

#[system(for_each)]
fn sample_movement(_: &SampleMovement, transform: &mut Transform, #[resource] time: &Time) {
    transform.set_position(vec3(
        time.seconds().cos() * 0.5,
        time.seconds().sin() * 0.5,
        2.0,
    ));
}
