use crate::{ecs, engine::EngineContext, math};

use super::uniform::Uniform;

/// A [`Transform`] component describes the position, rotation, and scale of an entity.
#[derive(Debug)]
pub struct Transform {
    position: math::Vec3,
    rotation: math::Quat,
    scale: math::Vec3,

    dirty: bool,
    transform: math::Mat4,
    transform_inv: math::Mat4,

    _buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

assert_impl_all!(Transform: ecs::storage::Component);

impl Transform {
    /// Creates a new [`Transform`].
    pub fn new(
        ctx: &EngineContext,
        position: math::Vec3,
        rotation: math::Quat,
        scale: math::Vec3,
    ) -> Self {
        let buffer = ctx.gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 2 * std::mem::size_of::<math::Mat4>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = ctx
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &ctx.gpu.default_bind_group_layouts.transform,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

        Self {
            position,
            rotation,
            scale,
            dirty: true,
            transform: math::Mat4::IDENTITY,
            transform_inv: math::Mat4::IDENTITY,
            _buffer: buffer,
            bind_group,
        }
    }

    /// Creates a new identity [`Transform`].
    pub fn identity(ctx: &EngineContext) -> Self {
        Self::new(ctx, math::Vec3::ZERO, math::Quat::IDENTITY, math::Vec3::ONE)
    }

    /// Returns the local position of the transform.
    pub fn position(&self) -> &math::Vec3 {
        &self.position
    }

    /// Sets the local position of the transform.
    pub fn set_position(&mut self, position: math::Vec3) {
        self.position = position;
        self.dirty = true;
    }

    /// Translates the transform by the given vector.
    pub fn translate(&mut self, translation: math::Vec3) {
        self.position += translation;
        self.dirty = true;
    }

    /// Returns the local rotation of the transform.
    pub fn rotation(&self) -> &math::Quat {
        &self.rotation
    }

    /// Sets the local rotation of the transform.
    pub fn set_rotation(&mut self, rotation: math::Quat) {
        self.rotation = rotation;
        self.dirty = true;
    }

    /// Rotates the transform by given euler angles.
    pub fn set_rotation_euler(&mut self, euler: math::Vec3) {
        self.rotation = math::Quat::from_euler(math::EulerRot::ZXY, euler.x, euler.y, euler.z);
        self.dirty = true;
    }

    /// Returns the local scale of the transform.
    pub fn scale(&self) -> &math::Vec3 {
        &self.scale
    }

    /// Sets the local scale of the transform.
    pub fn set_scale(&mut self, scale: math::Vec3) {
        self.scale = scale;
        self.dirty = true;
    }

    /// Returns the transformation matrix of the transform.
    pub fn transform(&self) -> &math::Mat4 {
        &self.transform
    }

    /// Returns the inverse transformation matrix of the transform.
    pub fn transform_inv(&self) -> &math::Mat4 {
        &self.transform_inv
    }

    /// Flushes the changes to the transformation matrix to the GPU.
    pub fn flush(&mut self, ctx: &EngineContext) {
        if !self.dirty {
            return;
        }

        self.transform =
            math::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position);
        self.transform_inv = self.transform.inverse();
        self.dirty = false;

        ctx.gpu.queue.write_buffer(
            &self._buffer,
            0,
            bytemuck::cast_slice(&[self.transform, self.transform_inv]),
        );
    }
}

impl Uniform for Transform {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
