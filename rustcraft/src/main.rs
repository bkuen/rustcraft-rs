//! Entry point and types/trait representing the
//! application/game.

#![feature(clamp)]

use crate::camera::PerspectiveCamera;
use crate::graphics::gl::{Gl, gl};
use crate::resources::Resources;
use crate::timestep::TimeStep;

use cgmath::{Vector3, Vector2};
use cgmath::num_traits::FromPrimitive;

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent, SwapInterval, OpenGlProfileHint, CursorMode};

use std::path::Path;
use std::sync::mpsc::Receiver;
use crate::world::chunk::ChunkRenderer;

pub mod camera;
pub mod entity;
pub mod input;
pub mod graphics;
pub mod resources;
pub mod timestep;
pub mod world;

struct WindowProps {
    height: i32,
    width: i32,
    fullscreen: bool,
    vsync: bool,
    polygon_mode: bool,
    title: &'static str,
}

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
    /// The window properties
    window_props: WindowProps,
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

        let window_props = WindowProps {
            width: 1080,
            height: 720,
            fullscreen: false,
            vsync: false,
            polygon_mode: false,
            title: "Rustcraft v0.1.0"
        };
        let (mut window, events) = Self::create_window(&glfw, &window_props);

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
            window_props,
            last_frame_time: 0.0,
        }
    }

    /// Create a new `GLFW` window with a title
    fn create_window(glfw: &Glfw, props: &WindowProps) -> (Window, Receiver<(f64, WindowEvent)>) {
        let (mut window, events) = glfw.create_window(props.width as u32, props.height as u32, props.title, glfw::WindowMode::Windowed)
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
        let mut camera = PerspectiveCamera::at_pos(Vector3::new(0.0, 34.0,  0.0));
        camera.rotate(45.0, -30.0, 0.0);

        let mut chunk_renderer: ChunkRenderer = ChunkRenderer::new(&self.gl, &resources);

        while !self.window.should_close() {
            let time = f32::from_f64(self.glfw.get_time()).unwrap();

            let time_step = TimeStep(time - self.last_frame_time);
            self.last_frame_time = time;

            chunk_renderer.add(Vector2::new(0.0, 0.0));

            // Render the scene
            chunk_renderer.clear();
            chunk_renderer.render(&camera);

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
                }

                if let glfw::WindowEvent::Key(Key::F5, _, Action::Press, _) = event {
                    self.window_props.polygon_mode = !self.window_props.polygon_mode;
                    if self.window_props.polygon_mode {
                        unsafe { self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
                    } else {
                        unsafe { self.gl.PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
                    }
                }

                if let glfw::WindowEvent::FramebufferSize(width, height) = event {
                    self.window_props.width = width;
                    self.window_props.height = height;
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