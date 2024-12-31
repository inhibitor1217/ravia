use crate::{ecs, engine::EngineContext};

use super::transform::Transform;

/// Attaches a system of the graphics engine.
pub fn system(builder: &mut ecs::systems::Builder) {
    builder.add_system(flush_transform_system());
}

#[ecs::system(for_each)]
#[filter(ecs::maybe_changed::<Transform>())]
fn flush_transform(transform: &mut Transform, #[resource] ctx: &EngineContext) {
    transform.flush(ctx);
}
