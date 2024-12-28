use super::{gpu, Vertex};

/// Specifies the vertex buffer configuration.
#[derive(Debug)]
pub struct VertexBufferConfig<'a> {
    attribute_formats: &'a [wgpu::VertexFormat],
}

impl VertexBufferConfig<'_> {
    /// Creates a new [`VertexBufferConfig`] from a vertex type.
    pub fn new<V: Vertex>() -> Self {
        Self {
            attribute_formats: V::ATTRIBUTE_FORMATS,
        }
    }
}

/// [`ShaderConfig`] holds the source, entry points and other configuration for a shader.
#[derive(Debug)]
pub struct ShaderConfig<'a> {
    pub name: Option<&'a str>,
    pub source: Option<wgpu::ShaderModuleDescriptor<'a>>,
    pub vertex_entry_point: &'static str,
    pub vertex_buffer: Option<VertexBufferConfig<'a>>,
    pub fragment_entry_point: &'static str,
}

impl<'a> ShaderConfig<'a> {
    /// Creates a new [`ShaderConfig`] from a WGSL shader source.
    pub fn new(source: &'a str) -> Self {
        Self {
            name: None,
            source: Some(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(source.into()),
            }),
            vertex_entry_point: "vs_main",
            vertex_buffer: None,
            fragment_entry_point: "fs_main",
        }
    }

    /// Adds a vertex buffer configuration to the shader config.
    pub fn with_vertex_buffer_config(mut self, config: VertexBufferConfig<'a>) -> Self {
        self.vertex_buffer = Some(config);
        self
    }
}

impl Default for ShaderConfig<'_> {
    fn default() -> Self {
        Self {
            name: None,
            source: None,
            vertex_entry_point: "vs_main",
            vertex_buffer: None,
            fragment_entry_point: "fs_main",
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
    pub fn new(gpu: &gpu::Gpu, config: ShaderConfig) -> Self {
        let surface_config = gpu.surface_config.lock().unwrap();

        let shader_module = gpu.device.create_shader_module(
            config
                .source
                .expect("Cannot create shader module without source"),
        );

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let mut vertex_buffer_attributes = vec![];
        let vertex_buffer_layout = {
            if let Some(vb_config) = config.vertex_buffer {
                let mut offset = 0;
                for (i, format) in vb_config.attribute_formats.iter().enumerate() {
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
            } else {
                vec![]
            }
        };

        let pipeline = gpu
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
