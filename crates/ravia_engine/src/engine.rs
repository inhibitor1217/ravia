use std::{future::Future, sync::Arc};

use log::{debug, info};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::Window,
};

use crate::graphics;

/// Engine configuration.
#[derive(Clone, Debug)]
pub struct EngineConfig {
    /// Window title.
    pub window_title: &'static str,
    /// Display width. Only effective in native mode.
    pub display_width: u32,
    /// Display height. Only effective in native mode.
    pub display_height: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "",
            display_width: 1024,
            display_height: 720,
        }
    }
}

/// Engine events to work with the winit event loop.
#[derive(Debug)]
enum EngineEvent {
    Initialized(Engine),
}

#[derive(Debug, Default)]
enum EngineState {
    #[default]
    Uninitialized,
    Created {
        config: EngineConfig,
        proxy: EventLoopProxy<EngineEvent>,
    },
    Running(Engine),
}

impl EngineState {
    /// Initializes the engine.
    /// If the engine is already initialized, this function will panic.
    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        match std::mem::take(self) {
            EngineState::Created { config, proxy } => {
                let window = Engine::new_window(event_loop, config.clone());
                resolve_future(async move {
                    let engine = Engine::new(window, config).await;
                    proxy
                        .send_event(EngineEvent::Initialized(engine))
                        .expect("Failed to send initialized event");
                })
            }
            EngineState::Running(_) => panic!("Engine already initialized"),
            EngineState::Uninitialized => panic!("Engine not initialized"),
        }
    }
}

impl ApplicationHandler<EngineEvent> for EngineState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        debug!(target: "ravia_engine::engine_state", "Engine resumed, engine = {:?}", self);

        if let EngineState::Created { .. } = self {
            self.initialize(event_loop);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: EngineEvent) {
        debug!(target: "ravia_engine::engine_state", "User event: {:?}", event);

        match event {
            EngineEvent::Initialized(engine) => {
                engine.request_frame();
                *self = EngineState::Running(engine);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        debug!(target: "ravia_engine::engine_state", "Window event: {:?}", event);

        let engine = match self {
            EngineState::Running(engine) => engine,
            _ => return,
        };

        if engine.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                engine.request_frame();
                engine.frame();
            }
            WindowEvent::Resized(physical_size) => {
                engine.resize(physical_size.width, physical_size.height);
            }
            WindowEvent::CloseRequested => {
                info!(target: "ravia_engine::engine_state", "Window close requested, exiting.");
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                info!(target: "ravia_engine::engine_state", "Window destroyed, exiting.");
                event_loop.exit();
            }
            _ => (),
        }
    }
}

/// [`Engine`] contains the resources for the components of the engine.
#[derive(Debug)]
pub struct Engine {
    window: Arc<Window>,
    gpu: Arc<graphics::Gpu>,
}

impl Engine {
    /// Initializes and runs the main event loop for the engine.    
    pub fn run(config: EngineConfig) {
        let event_loop = EventLoop::<EngineEvent>::with_user_event()
            .build()
            .expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        let engine_state = EngineState::Created {
            config,
            proxy: event_loop.create_proxy(),
        };

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::EventLoopExtWebSys;
            event_loop.spawn_app(engine_state);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut engine_state = engine_state;
            event_loop
                .run_app(&mut engine_state)
                .expect("Failed to run main event loop");
        }
    }

    /// Creates a new [`Engine`].
    async fn new(window: Window, _config: EngineConfig) -> Self {
        let window = Arc::new(window);

        debug!(target: "ravia_engine::engine", "Initializing WebGPU resources");
        let gpu = graphics::Gpu::new(window.clone()).await;
        let gpu = Arc::new(gpu);

        Self { window, gpu }
    }

    /// Creates a new [`Window`].
    fn new_window(event_loop: &ActiveEventLoop, config: EngineConfig) -> Window {
        let window_attrs = Window::default_attributes()
            .with_title(config.window_title)
            .with_inner_size(LogicalSize::new(
                config.display_width,
                config.display_height,
            ));

        let window = event_loop
            .create_window(window_attrs)
            .expect("Failed to create window");

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;

            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| {
                    let root = doc.get_element_by_id("root")?;
                    let canvas = web_sys::Element::from(window.canvas()?);
                    root.append_child(&canvas).ok()?;
                    Some(())
                })
                .expect("Failed to append canvas to root element");
        }

        window
    }

    /// Handles the display resize.
    fn resize(&self, width: u32, height: u32) {
        self.gpu.resize(width, height);
    }

    /// Requests a new frame.
    fn request_frame(&self) {
        self.window.request_redraw();
    }

    /// Handles the single frame render.
    fn frame(&self) {
        self.gpu.render();
    }
}

fn resolve_future<F: Future<Output = ()> + 'static>(f: F) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(f);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(f);
    }
}
