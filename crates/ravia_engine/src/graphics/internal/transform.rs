use wgpu::util::DeviceExt;

use crate::{ecs, engine::EngineContext, math};

use super::uniform::Uniform;

/// A [`Transform`] component describes the position, rotation, and scale of an entity.
#[derive(Debug)]
pub struct Transform {
    pub transform: math::Mat4,

    _buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

assert_impl_all!(Transform: ecs::storage::Component);

impl Transform {
    /// Creates a new [`Transform`] from a transformation matrix.
    pub fn new(ctx: &EngineContext, transform: math::Mat4, camera: bool) -> Self {
        let transform = if camera {
            transform.inverse()
        } else {
            transform
        };

        let buffer = ctx
            .gpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[transform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let bind_group_layout = if camera {
            &ctx.gpu.default_bind_group_layouts.camera_transform
        } else {
            &ctx.gpu.default_bind_group_layouts.model_transform
        };

        let bind_group = ctx
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        Self {
            transform,
            _buffer: buffer,
            bind_group,
        }
    }

    /// Creates a new identity [`Transform`].
    pub fn identity(ctx: &EngineContext, camera: bool) -> Self {
        Self::new(ctx, math::Mat4::IDENTITY, camera)
    }
}

impl Uniform for Transform {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
