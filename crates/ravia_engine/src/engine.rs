use std::sync::Arc;

use log::{debug, info, trace};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

use crate::graphics;

/// Engine configuration.
#[derive(Default)]
pub struct EngineConfig {
    /// Window title.
    pub window_title: &'static str,
}

enum EngineState {
    Uninitialized(EngineConfig),
    Initialized(Engine),
}

impl EngineState {
    /// Initializes the engine.
    /// If the engine is already initialized, this function will panic.
    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        match self {
            EngineState::Uninitialized(config) => {
                *self = EngineState::Initialized(Engine::new(event_loop, config));
            }
            EngineState::Initialized(_) => panic!("Engine already initialized"),
        }
    }
}

impl ApplicationHandler for EngineState {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        match self {
            EngineState::Uninitialized(_) => self.initialize(event_loop),
            EngineState::Initialized(_) => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        trace!(target: "ravia_engine", "Window event: {:?}", event);

        let app = match self {
            EngineState::Uninitialized(_) => return,
            EngineState::Initialized(app) => app,
        };

        if app.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                info!(target: "ravia_engine", "Window close requested, exiting.");
                event_loop.exit();
            }
            WindowEvent::Destroyed => {
                info!(target: "ravia_engine", "Window destroyed, exiting.");
                event_loop.exit();
            }
            _ => (),
        }
    }
}

/// [`Engine`] contains the resources for the components of the engine.
pub struct Engine {
    window: Arc<Window>,
    gpu: Arc<graphics::Gpu>,
}

impl Engine {
    /// Initializes and runs the main event loop for the engine.
    pub fn run(config: EngineConfig) {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut engine_state = EngineState::Uninitialized(config);
        event_loop
            .run_app(&mut engine_state)
            .expect("Failed to run main event loop");
    }

    /// Creates a new [`Engine`].
    fn new(event_loop: &ActiveEventLoop, config: &EngineConfig) -> Self {
        debug!(target: "ravia_engine", "Initializing window");
        let window = Self::init_window(event_loop, config);
        let window = Arc::new(window);

        debug!(target: "ravia_engine", "Initializing WebGPU resources");
        let gpu = pollster::block_on(graphics::Gpu::new(window.clone()));
        let gpu = Arc::new(gpu);

        Self { window, gpu }
    }

    fn init_window(event_loop: &ActiveEventLoop, config: &EngineConfig) -> Window {
        let window_attrs = Window::default_attributes().with_title(config.window_title);

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
}
