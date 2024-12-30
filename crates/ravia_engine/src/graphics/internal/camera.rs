use crate::{ecs, engine::EngineContext, math};

/// A [`Camera`] is used to render the scene from a specific point of view.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// The projection matrix of the camera.
    pub projection: math::Mat4,
}

assert_impl_all!(Camera: ecs::storage::Component);

impl Camera {
    /// Creates a no-op [`Camera`].
    pub fn noop() -> Self {
        Self {
            projection: math::Mat4::IDENTITY,
        }
    }

    /// Creates a perspective [`Camera`].
    pub fn perspective(fov_y: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            projection: math::Mat4::perspective_lh(fov_y, aspect_ratio, z_near, z_far),
        }
    }

    /// Creates a perspective [`Camera`] with the default parameters.
    pub fn perspective_with_defaults(ctx: &EngineContext) -> Self {
        let surface_config = ctx.gpu.surface_config.lock().unwrap();
        let width = surface_config.width as f32;
        let height = surface_config.height as f32;
        Self::perspective(45.0, width / height, 0.1, 100.0)
    }
}
