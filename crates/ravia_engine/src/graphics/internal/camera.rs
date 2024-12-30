use wgpu::util::DeviceExt;

use crate::{ecs, engine::EngineContext, math};

use super::uniform::Uniform;

/// A [`Camera`] is used to render the scene from a specific point of view.
#[derive(Debug)]
pub struct Camera {
    /// The projection matrix of the camera.
    pub projection: math::Mat4,

    _buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

assert_impl_all!(Camera: ecs::storage::Component);

impl Camera {
    fn new(ctx: &EngineContext, projection: math::Mat4) -> Self {
        let buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[projection]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group = ctx
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &ctx.gpu.default_bind_group_layouts.camera,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        Self {
            projection,
            _buffer: buffer,
            bind_group,
        }
    }

    /// Creates a no-op [`Camera`].
    pub fn noop(ctx: &EngineContext) -> Self {
        Self::new(ctx, math::Mat4::IDENTITY)
    }

    /// Creates a perspective [`Camera`].
    pub fn perspective(
        ctx: &EngineContext,
        fov_y: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Self {
        Self::new(
            ctx,
            math::Mat4::perspective_lh(fov_y, aspect_ratio, z_near, z_far),
        )
    }

    /// Creates a perspective [`Camera`] with the default parameters.
    pub fn perspective_with_defaults(ctx: &EngineContext) -> Self {
        let surface_config = ctx.gpu.surface_config.lock().unwrap();
        let width = surface_config.width as f32;
        let height = surface_config.height as f32;
        Self::perspective(ctx, 45.0, width / height, 0.1, 100.0)
    }
}

impl Uniform for Camera {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
