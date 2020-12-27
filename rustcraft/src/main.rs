//! Entry point and types/trait representing the
//! application/game.

#![feature(clamp)]

use crate::camera::PerspectiveCamera;
use crate::graphics::gl::{Gl, gl};
use crate::resources::Resources;
use crate::timestep::TimeStep;
use crate::world::block::CubeRenderer;

use cgmath::{Vector3};
use cgmath::num_traits::FromPrimitive;

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent, SwapInterval, OpenGlProfileHint, CursorMode};

use std::path::Path;
use std::sync::mpsc::Receiver;

pub mod camera;
pub mod entity;
pub mod input;
pub mod graphics;
pub mod resources;
pub mod timestep;
pub mod world;

/// Rustcraft
///
/// The `Rustcraft` struct represents the main
/// application. It provides all game related
/// functionality like `window creation`, `game loop`
/// and `rendering`.
struct Rustcraft {
    /// A `OpenGL` 'instance'
    gl: Gl,
    /// A `GLFW` instance
    glfw: Glfw,
    /// An `GLFW` event receiver
    events: Receiver<(f64, WindowEvent)>,
    /// A `GLFW` window,
    window: Window,
    /// The last frame time
    last_frame_time: f32,
}

impl Rustcraft {
    /// Initialize a new `Rustcraft` application
    /// by creating an event loop, a window and
    /// an `OpenGL` instance/context.
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
        glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        let (mut window, events) = Self::create_window(&glfw);

        let (width, height) = window.get_size();

        window.set_cursor_mode(CursorMode::Disabled);
        window.set_cursor_pos(width as f64 / 2.0, height as f64 / 2.0);

        let gl = Gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);

        unsafe {
            gl.ClearColor(0.23, 0.38, 0.47, 1.0);
            gl.Viewport(0, 0, width, height);
        }


        Self {
            glfw,
            gl,
            events,
            window,
            last_frame_time: 0.0,
        }
    }

    /// Create a new `GLFW` window with a title
    fn create_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
        let (mut window, events) = glfw.create_window(1080, 720, "", glfw::WindowMode::Windowed)
            .expect("Failed to create window.");

        window.make_current();
        window.set_all_polling(true);

        (window, events)
    }

    /// Run the main game loop of `Rustcraft`
    fn run(&mut self) {
        self.glfw.set_swap_interval(SwapInterval::Sync(1));

        unsafe {
            self.gl.Enable(gl::BLEND);
            self.gl.Enable(gl::DEPTH_TEST);
            self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let resources = Resources::from_relative_exe_path(Path::new("res")).unwrap();
        let mut camera = PerspectiveCamera::at_pos(Vector3::new(0.0, 0.0,  5.0));
        camera.set_pos(Vector3::new(0f32, 2f32, 0f32));
        camera.look_at(Vector3::new(0f32, 0f32, -4f32));

        let mut cube_renderer = CubeRenderer::new(&self.gl, &resources);

        while !self.window.should_close() {
            let time = f32::from_f64(self.glfw.get_time()).unwrap();

            let time_step = TimeStep(time - self.last_frame_time);
            self.last_frame_time = time;

            cube_renderer.add(Vector3::new(0.0, 0.0, 4.0));
            cube_renderer.add(Vector3::new(0.0, 0.0, 5.0));

            // Render scene
            cube_renderer.clear();
            cube_renderer.render(&camera);

            // Swap front and back buffers
            self.window.swap_buffers();

            // Poll for and process events
            self.glfw.poll_events();

            // Handle player input
            input::handle_mouse_input(&mut self.window, &mut camera);
            input::handle_key_input(time_step, &self.window, &mut camera);

            for (_, event) in glfw::flush_messages(&self.events) {

                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.window.set_should_close(true);
                    return;
                }

                if let glfw::WindowEvent::FramebufferSize(width, height) = event {
                    unsafe { self.gl.Viewport(0, 0, width, height); }
                    camera.set_aspect_ratio((width / height) as f32);
                }
            }
        }
    }
}

/// The entry function of this binary
fn main() {
    let mut rustcraft = Rustcraft::new();
    rustcraft.run();
}