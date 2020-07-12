//! Entry point and types/trait representing the
//! application/game.

use std::sync::Arc;
use vulkano::{
    instance::{ApplicationInfo, Instance, Version}
};
use winit::{
    dpi::{LogicalSize},
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};
use winit::platform::desktop::EventLoopExtDesktop;
use vulkano_win::VkSurfaceBuild;
use winit::window::Window;
use vulkano::swapchain::Surface;

/// RustcraftApplication
///
/// This struct represents the application and
/// the main game loop.
struct RustcraftApplication {
    /// A event loop which is subscribable
    events_loop: EventLoop<()>,
    /// A Vulkan instance
    instance: Arc<Instance>,
    /// A window surface
    window: Arc<Surface<Window>>,
}

impl RustcraftApplication {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    /// Instantiate a new `RustCraftApplication`
    pub fn new() -> Self {
        let instance = Self::create_instance();
        let events_loop = EventLoop::new();
        let window = Self::create_window(&events_loop, instance.clone());
        Self {
            events_loop,
            instance,
            window
        }
    }

    /// Initialize a new Vulkan instance and returns it.
    /// If an error occures, this function will panic.
    fn create_instance() -> Arc<Instance> {
        let app_info = ApplicationInfo {
            application_name: Some("Rustcraft".into()),
            application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
            engine_name: Some("No Engine".into()),
            engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        };

        let required_extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &required_extensions, None)
            .expect("Failed to create Vulkan instance")
    }

    /// Initialize a new `Winit` Window and return
    /// its event loop.
    fn create_window(events_loop: &EventLoop<()>, instance: Arc<Instance>) -> Arc<Surface<Window>> {
        let window = WindowBuilder::new()
            .with_title("Rustcraft 0.1.0")
            .with_inner_size(LogicalSize::new(
                RustcraftApplication::WIDTH,
                RustcraftApplication::HEIGHT
            ))
            .build_vk_surface(events_loop, instance).unwrap();

        window
    }

    /// This function subscribes to the event loop and keeps
    /// the window alive.
    fn run(self) {
        self.events_loop.run(|event, _, control_flow| {
            match event {
                Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                    *control_flow = ControlFlow::Exit;
                },
                _ => ()
            }
        });
    }
}

/// The entry point of the `Rustcraft` application.
fn main() {
    let application = RustcraftApplication::new();
    application.run();
}