use std::ops::Deref;

use wgpu::util::DeviceExt;

use crate::engine::EngineContext;

use super::uniform::{Uniform, UniformType};

/// Filter mode for the texture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilterMode {
    /// Nearest neighbor sampling.
    Point,
    /// Bilinear interpolation in uv space.
    Bilinear,
    /// Trilinear interpolation in uv space and mipmap levels.
    Trilinear,
}

impl Default for TextureFilterMode {
    fn default() -> Self {
        Self::Bilinear
    }
}

impl TextureFilterMode {
    fn mag_filter(&self) -> wgpu::FilterMode {
        match self {
            Self::Point => wgpu::FilterMode::Nearest,
            Self::Bilinear => wgpu::FilterMode::Linear,
            Self::Trilinear => wgpu::FilterMode::Linear,
        }
    }

    fn min_filter(&self) -> wgpu::FilterMode {
        match self {
            Self::Point => wgpu::FilterMode::Nearest,
            Self::Bilinear => wgpu::FilterMode::Linear,
            Self::Trilinear => wgpu::FilterMode::Linear,
        }
    }

    fn mipmap_filter(&self) -> wgpu::FilterMode {
        match self {
            Self::Point => wgpu::FilterMode::Nearest,
            Self::Bilinear => wgpu::FilterMode::Nearest,
            Self::Trilinear => wgpu::FilterMode::Linear,
        }
    }
}

/// [`Texture`] contains the WebGPU texture and its underlying resources, and abind group.
#[derive(Debug)]
pub struct Texture {
    _texture: wgpu::Texture,
    _texture_view: wgpu::TextureView,
    _sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,
    filter_mode: TextureFilterMode,
    uniform_type: UniformType,
}

impl Texture {
    /// Creates a new 2D [`Texture`].
    pub fn new_2d<D: Deref<Target = [u8]>>(
        ctx: &EngineContext,
        size: (u32, u32),
        data: D,
        filter_mode: TextureFilterMode,
    ) -> Self {
        let (width, height) = size;

        let texture = ctx.gpu.device.create_texture_with_data(
            &ctx.gpu.queue,
            &wgpu::TextureDescriptor {
                size: wgpu::Extent3d {
                    width,
                    height,
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
            &data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = ctx.gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: filter_mode.mag_filter(),
            min_filter: filter_mode.min_filter(),
            mipmap_filter: filter_mode.mipmap_filter(),
            ..Default::default()
        });

        let bind_group = ctx
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &ctx.gpu.default_bind_group_layouts.texture_2d,
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
            _texture: texture,
            _texture_view: texture_view,
            _sampler: sampler,
            bind_group,
            filter_mode,
            uniform_type: UniformType::Texture2D,
        }
    }

    /// Creates a default 2D [`Texture`] with a checkerboard pattern.
    pub fn default_2d(ctx: &EngineContext) -> Self {
        const BRIGHT: u8 = 200;
        const DARK: u8 = 80;
        const ALPHA: u8 = 255;

        let (width, height) = (8, 8);
        let mut data = vec![0; width * height * 4];
        for i in 0..height {
            for j in 0..width {
                let use_color = ((i + j) % 2) > 0;
                data[i * width * 4 + j * 4] = if use_color { BRIGHT } else { DARK };
                data[i * width * 4 + j * 4 + 1] = if use_color { BRIGHT } else { DARK };
                data[i * width * 4 + j * 4 + 2] = if use_color { BRIGHT } else { DARK };
                data[i * width * 4 + j * 4 + 3] = ALPHA;
            }
        }

        Self::new_2d(
            ctx,
            (width as u32, height as u32),
            data,
            TextureFilterMode::Point,
        )
    }

    /// Returns the filter mode for the texture.
    pub fn filter_mode(&self) -> TextureFilterMode {
        self.filter_mode
    }

    /// Sets the filter mode for the texture.
    pub fn set_filter_mode(&mut self, ctx: &EngineContext, filter_mode: TextureFilterMode) {
        if self.filter_mode == filter_mode {
            return;
        }

        self.filter_mode = filter_mode;

        self._sampler = ctx.gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: filter_mode.mag_filter(),
            min_filter: filter_mode.min_filter(),
            mipmap_filter: filter_mode.mipmap_filter(),
            ..Default::default()
        });

        self.bind_group = ctx
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &ctx.gpu.default_bind_group_layouts.texture_2d,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&self._texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&self._sampler),
                    },
                ],
                label: None,
            });
    }
}

impl Texture {
    pub(super) const TEXTURE_2D_BIND_GROUP_LAYOUT_ENTRIES: &[wgpu::BindGroupLayoutEntry] = &[
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                multisampled: false,
            },
            count: None,
        },
        wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        },
    ];
}

impl Uniform for Texture {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn uniform_type(&self) -> UniformType {
        self.uniform_type
    }
}
