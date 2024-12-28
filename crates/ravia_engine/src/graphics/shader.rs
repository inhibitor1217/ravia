use super::{gpu, Vertex};

/// [`ShaderConfig`] holds the source, entry points and other configuration for a shader.
#[derive(Debug)]
pub struct ShaderConfig<'a> {
    pub name: Option<&'a str>,
    pub source: Option<wgpu::ShaderModuleDescriptor<'a>>,
    pub vertex_entry_point: &'static str,
    pub fragment_entry_point: &'static str,
}

/// Creates a [`ShaderConfig`] from a WGSL shader source file.
#[macro_export]
macro_rules! shader_config {
    ($path:literal) => {
        ShaderConfig::new(include_str!($path))
    };
    ($path:literal, $name:expr) => {{
        let mut config = ShaderConfig::new(include_str!($path));
        config.name = Some($name);
        config
    }};
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
            fragment_entry_point: "fs_main",
        }
    }
}

impl Default for ShaderConfig<'_> {
    fn default() -> Self {
        Self {
            name: None,
            source: None,
            vertex_entry_point: "vs_main",
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

        let render_pipeline_layout =
            gpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let render_pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: Some(config.vertex_entry_point),
                    // FIXME: temporary vertex buffer
                    buffers: &[Vertex::desc()],
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

        Self {
            pipeline: render_pipeline,
        }
    }

    /// Returns the underlying [`wgpu::RenderPipeline`].
    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn can_load_shader_config_from_file() {
        let config = crate::shader_config!("shaders/triangle.wgsl", "triangle");

        assert_eq!(config.name, Some("triangle"));
        assert!(config.source.is_some());
        assert_eq!(config.vertex_entry_point, "vs_main");
        assert_eq!(config.fragment_entry_point, "fs_main");
    }
}
