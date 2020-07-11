use vulkano::{
    instance::{Instance, ApplicationInfo}
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder
};

fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance.")
    };

    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
    surface.window().set_title("Rustcraft 0.1.0");

    events_loop.run(|event, _, control_flow| {
        match event {
            Event::WindowEvent { event: winit::event::WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}