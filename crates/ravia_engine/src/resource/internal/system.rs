use crate::{
    ecs::{self, systems::CommandBuffer, Entity},
    engine::EngineContext,
    graphics::load_mesh_from_obj,
};

use super::{resource::Resource, resource_manager::ResourceState};

/// Attaches a system of the resource engine.
pub fn system(builder: &mut ecs::systems::Builder) {
    builder.add_system(request_resource_system());
    builder.add_system(bind_mesh_system());
}

#[ecs::system(for_each)]
fn request_resource(resource: &mut Resource, #[resource] ctx: &EngineContext) {
    if !resource.should_request() {
        return;
    }

    ctx.resource_manager.request(resource);
}

#[ecs::system(for_each)]
fn bind_mesh(
    cmd: &mut CommandBuffer,
    #[resource] ctx: &EngineContext,
    entity: &Entity,
    resource: &Resource,
) {
    if resource.should_request() {
        return;
    }

    if let ResourceState::Loaded(data) = ctx.resource_manager.get(resource.key.unwrap()) {
        if let Ok(mesh) = load_mesh_from_obj(ctx, &data) {
            cmd.add_component(entity.clone(), mesh);
        }
    }
}
