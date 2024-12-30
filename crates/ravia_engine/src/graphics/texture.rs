use std::ops::Deref;

use wgpu::util::DeviceExt;

use super::gpu;

/// Configuration to create a [`Texture2D`].
pub struct Texture2DConfig<D: Deref<Target = [u8]>> {
    pub size: (u32, u32),
    pub data: D,
}

/// A 2D texture.
pub struct Texture2D {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
}

impl Texture2D {
    /// Creates a new [`Texture2D`].
    pub fn new<D: Deref<Target = [u8]>>(gpu: &gpu::Gpu, config: Texture2DConfig<D>) -> Self {
        let texture = gpu.device.create_texture_with_data(
            &gpu.queue,
            &wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width: config.size.0,
                    height: config.size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: None,
                view_formats: &[],
            },
            Default::default(),
            &config.data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &gpu.asset().default_texture_2d_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: None,
        });

        Self {
            texture,
            texture_view,
            sampler,
            bind_group,
        }
    }
}
