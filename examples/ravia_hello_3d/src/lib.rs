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
struct ExampleMovement {}

fn init_world(world: &mut World, ctx: &EngineContext) {
    let camera = Camera::perspective_with_defaults(ctx);
    world.push((camera, Transform::identity(ctx)));

    let cube_obj = ravia_utils::read_resource("engine/model/cube.obj").unwrap();
    let mesh = ravia_utils::load_mesh_from_obj(ctx, cube_obj).unwrap();

    let mut material = Material::new(
        ctx,
        &ShaderConfig::new(include_str!("triangle_tex_perspective.wgsl"))
            .with_vertex_type::<Vertex3DStandard>()
            .with_uniforms(&[
                UniformType::Texture2D,
                UniformType::Camera,
                UniformType::CameraTransform,
                UniformType::ModelTransform,
            ]),
    );
    let texture = Texture::default_2d(ctx);
    material.texture = Some(texture);

    world.push((mesh, material, Transform::identity(ctx), ExampleMovement {}));
}

fn init_system(builder: &mut systems::Builder) {
    builder.add_system(example_movement_system());
}

#[system(for_each)]
fn example_movement(_: &ExampleMovement, transform: &mut Transform, #[resource] time: &Time) {
    transform.set_position(vec3(
        time.seconds().cos() * 0.5,
        time.seconds().sin() * 0.5,
        -5.0,
    ));

    transform.set_rotation_euler(time.seconds() * Vec3::ONE);
}
