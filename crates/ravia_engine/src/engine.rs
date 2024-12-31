use core::fmt;
use std::{future::Future, sync::Arc};

use log::{debug, info};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::Window,
};

use crate::{ecs, graphics, math, time};

/// World initializer.
pub type InitWorld = fn(&mut ecs::World, &EngineContext);

/// User system initializer.
pub type InitSystem = fn(&mut ecs::systems::Builder);

/// Engine configuration.
#[derive(Clone, Copy, Debug)]
pub struct EngineConfig {
    /// Window title.
    pub window_title: &'static str,
    /// Display size. Only effective in native mode.
    pub display_size: math::UVec2,
    /// World initializer.
    pub init_world: InitWorld,
    /// User system initializer.
    pub init_system: InitSystem,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_title: "",
            display_size: math::uvec2(1024, 720),
            init_world: |_, _| {},
            init_system: |_| {},
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
                let window = Engine::new_window(event_loop, config);
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
                engine.resize(math::uvec2(physical_size.width, physical_size.height));
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
pub struct Engine {
    world: ecs::World,
    resources: ecs::Resources,
    schedule: ecs::Schedule,

    window: Arc<Window>,
    gpu: Arc<graphics::Gpu>,
    timer: time::Timer,
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
    async fn new(window: Window, config: EngineConfig) -> Self {
        let window = Arc::new(window);

        debug!(target: "ravia_engine::engine", "Initializing WebGPU resources");
        let gpu = graphics::Gpu::new(window.clone()).await;
        let gpu = Arc::new(gpu);

        let timer = time::Timer::new();

        let mut world = ecs::World::default();

        let mut resources = ecs::Resources::default();
        resources.insert(EngineContext { gpu: gpu.clone() });

        let mut schedule_builder = ecs::Schedule::builder();
        graphics::system(&mut schedule_builder);
        (config.init_system)(&mut schedule_builder);
        let schedule = schedule_builder.build();

        (config.init_world)(&mut world, &EngineContext { gpu: gpu.clone() });

        Self {
            world,
            resources,
            schedule,

            window,
            gpu,
            timer,
        }
    }

    /// Creates a new [`Window`].
    fn new_window(event_loop: &ActiveEventLoop, config: EngineConfig) -> Window {
        let window_attrs = Window::default_attributes()
            .with_title(config.window_title)
            .with_inner_size(LogicalSize::new(
                config.display_size.x,
                config.display_size.y,
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
    fn resize(&self, size: math::UVec2) {
        self.gpu.resize(size);
    }

    /// Requests a new frame.
    fn request_frame(&self) {
        self.window.request_redraw();
    }

    /// Handles the single frame update.
    fn frame(&mut self) {
        self.timer.frame();
        let time = self.timer.time();
        self.resources.insert(time);

        self.schedule.execute(&mut self.world, &mut self.resources);
        self.gpu.render(&self.world);
    }
}

impl fmt::Debug for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ravia Engine")
    }
}

/// [`EngineContext`] contains the reference for the global resources, which can be then accessed
/// by the system update loop.
#[derive(Debug)]
pub struct EngineContext {
    pub gpu: Arc<graphics::Gpu>,
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
