use crate::engine::EngineContext;

use super::{UniformType, Vertex};

/// [`ShaderConfig`] holds the source, entry points and other configuration for a shader.
#[derive(Clone, Copy, Debug)]
pub struct ShaderConfig<'a> {
    source: &'a str,
    vertex_entry_point: &'static str,
    vertex_attribute_formats: &'a [wgpu::VertexFormat],
    fragment_entry_point: &'static str,
    uniforms: &'a [UniformType],
}

impl<'a> ShaderConfig<'a> {
    /// Creates a new [`ShaderConfig`] from a WGSL shader source.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            vertex_entry_point: "vs_main",
            vertex_attribute_formats: &[],
            fragment_entry_point: "fs_main",
            uniforms: &[],
        }
    }

    /// Specifies the vertex type.
    pub fn with_vertex_type<V: Vertex>(mut self) -> Self {
        self.vertex_attribute_formats = V::ATTRIBUTE_FORMATS;
        self
    }

    /// Specifies the uniforms.
    pub fn with_uniforms(mut self, uniforms: &'a [UniformType]) -> Self {
        self.uniforms = uniforms;
        self
    }
}

impl Default for ShaderConfig<'_> {
    fn default() -> Self {
        Self {
            source: "",
            vertex_entry_point: "vs_main",
            vertex_attribute_formats: &[],
            fragment_entry_point: "fs_main",
            uniforms: &[],
        }
    }
}

/// Holds a compiled shader and underlying rendering pipeline.
#[derive(Debug)]
pub struct Shader {
    pipeline: wgpu::RenderPipeline,
}

impl Shader {
    /// Creates a new [`Shader`].
    pub fn new(ctx: &EngineContext, config: &ShaderConfig) -> Self {
        let surface_config = ctx.gpu.surface_config.lock().unwrap();

        let shader_module = ctx
            .gpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(config.source.into()),
            });

        let pipeline_layout =
            ctx.gpu
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: config
                        .uniforms
                        .iter()
                        .map(|uniform_type| {
                            ctx.gpu
                                .default_bind_group_layouts
                                .uniform_layout(uniform_type)
                        })
                        .collect::<Vec<_>>()
                        .as_slice(),
                    push_constant_ranges: &[],
                });

        let mut vertex_buffer_attributes = vec![];
        let vertex_buffer_layout = {
            let mut offset = 0;
            for (i, format) in config.vertex_attribute_formats.iter().enumerate() {
                vertex_buffer_attributes.push(wgpu::VertexAttribute {
                    format: *format,
                    offset,
                    shader_location: i as u32,
                });
                offset += format.size();
            }

            vec![wgpu::VertexBufferLayout {
                array_stride: offset,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &vertex_buffer_attributes,
            }]
        };

        let pipeline = ctx
            .gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: Some(config.vertex_entry_point),
                    buffers: &vertex_buffer_layout,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: Some(config.fragment_entry_point),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
                cache: None,
            });

        Self { pipeline }
    }

    /// Returns the underlying [`wgpu::RenderPipeline`].
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}
