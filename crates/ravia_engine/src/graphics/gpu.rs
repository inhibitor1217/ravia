use std::sync::{Arc, Mutex};

use log::{error, info};

use crate::ecs::{self, IntoQuery};

use super::{Mesh, Shader, ShaderConfig, Vertex2DColor, VertexBufferConfig};

/// [`Gpu`] holds the WebGPU device and its resources.
#[derive(Debug)]
pub struct Gpu {
    /// A WebGPU device.
    pub device: wgpu::Device,

    /// Handle for a WebGPU command queue.
    pub queue: wgpu::Queue,

    /// A WebGPU surface. Typically this will be a render target.
    ///
    /// A surface corresponds to a platform-specific window (e.g. a canvas in web platforms).
    /// The window lives during the whole engine lifetime, so it holds a static lifetime.
    pub surface: wgpu::Surface<'static>,

    /// A WebGPU surface configuration.
    pub surface_config: Mutex<wgpu::SurfaceConfiguration>,

    /// A window handle.
    pub window: Arc<winit::window::Window>,

    /// A collection of GPU assets that are loaded on initialization.
    asset: Option<GpuAsset>,
}

impl Gpu {
    /// Creates a new [`Gpu`] and initializes its resources.
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        let instance = wgpu::Instance::new(Default::default());

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create wgpu surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to request wgpu adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("ravia_engine"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to request wgpu device");

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);
        let (surface_width, surface_height) = Self::window_size(&window);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: surface_width,
            height: surface_height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let mut gpu = Self {
            device,
            queue,
            surface,
            surface_config: Mutex::new(surface_config),
            window,
            asset: None,
        };

        gpu.asset = Some(GpuAsset::load(&gpu));

        gpu
    }

    /// Retrieves the current display size from the window.
    pub fn window_size(window: &winit::window::Window) -> (u32, u32) {
        let winit::dpi::PhysicalSize { width, height } = window.inner_size();
        (width.max(1), height.max(1))
    }

    /// Resizes the GPU resources to match the window size.
    pub fn resize(&self, width: u32, height: u32) {
        let mut surface_config = self.surface_config.lock().unwrap();
        surface_config.width = width.max(1);
        surface_config.height = height.max(1);
        self.surface.configure(&self.device, &surface_config);
    }

    /// Renders the current frame.
    ///
    /// For now, this procedure contains all the details about wgpu render pipeline specific to
    /// surface texture. We hope to move this to a separate module in the future.
    pub fn render(&self, world: &mut ecs::World) {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                info!(target: "ravia_engine::graphics::gpu", "Surface lost or outdated, resizing");

                let (w, h) = Self::window_size(&self.window);
                self.resize(w, h);
                self.surface
                    .get_current_texture()
                    .expect("Failed to get current surface texture")
            }
            Err(wgpu::SurfaceError::Timeout) => {
                error!(target: "ravia_engine::graphics::gpu", "Surface timeout, skipping frame");
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                error!(target: "ravia_engine::graphics::gpu", "Out of memory, skipping frame");
                return;
            }
        };

        let target_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("ravia_engine"),
                });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("ravia_engine"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let asset = self
                .asset
                .as_ref()
                .expect("GPU asset not loaded, proper initialization is required");

            let mut renderables_query = <&mut Mesh<Vertex2DColor>>::query();

            // For now, we simply iterate over all the renderable components and render them in separate draw calls.
            // Later we will optimize this by allocating a single buffer for multiple meshes and using a single draw call.
            render_pass.set_pipeline(asset.default_shader.pipeline());
            for mesh in renderables_query.iter_mut(world) {
                let buffers = mesh.allocate_buffers(self);
                render_pass.set_vertex_buffer(0, buffers.vertex_slice());
                render_pass.set_index_buffer(buffers.index_slice(), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(mesh.indices(), 0, 0..1);
            }
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
    }
}

/// A collection of resources to be loaded for GPU.
#[derive(Debug)]
struct GpuAsset {
    /// Default shader to use for the rendering pipeline.
    default_shader: Shader,
}

impl GpuAsset {
    /// Loads the GPU assets.
    pub fn load(gpu: &Gpu) -> Self {
        Self {
            default_shader: Shader::new(
                gpu,
                // For now, we use a simple shader that draws a triangle.
                ShaderConfig::new(include_str!("shaders/triangle.wgsl"))
                    .with_vertex_buffer_config(VertexBufferConfig::new::<Vertex2DColor>()),
            ),
        }
    }
}
