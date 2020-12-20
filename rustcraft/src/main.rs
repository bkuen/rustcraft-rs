//! Entry point and types/trait representing the
//! application/game.

#![feature(clamp)]

use crate::graphics::buffer::{VertexBuffer, IndexBuffer, VertexArray, VertexBufferLayout};
use crate::graphics::gl::{Gl, gl, types::*};
use crate::graphics::shader::{ShaderProgram};
use crate::graphics::renderer::Renderer;
use crate::graphics::texture::Texture;
use crate::resources::Resources;
use cgmath::{Vector4, Matrix4, SquareMatrix, Vector3};
use glfw::{Action, Context, Key, Glfw, Window, WindowEvent, SwapInterval, OpenGlProfileHint, CursorMode};
use std::mem::size_of;
use std::path::Path;
use std::sync::mpsc::Receiver;
use crate::timestep::TimeStep;
use crate::camera::PerspectiveCamera;
use cgmath::num_traits::FromPrimitive;

pub mod camera;
pub mod input;
pub mod graphics;
pub mod resources;
pub mod timestep;

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


        let cube_vertices: [f32; 40] = [
            // front
            -1.0, -1.0,  1.0, 0.0, 0.0,
             1.0, -1.0,  1.0, 1.0, 0.0,
             1.0,  1.0,  1.0, 1.0, 1.0,
            -1.0,  1.0,  1.0, 0.0, 1.0,
            // back
            -1.0, -1.0, -1.0, 0.0, 0.0,
             1.0, -1.0, -1.0, 1.0, 0.0,
             1.0,  1.0, -1.0, 1.0, 1.0,
            -1.0,  1.0, -1.0, 0.0, 1.0,
        ];

        let cube_indices: [u32; 36] = [
            // front
            0, 1, 2,
            2, 3, 0,
            // right
            1, 5, 6,
            6, 2, 1,
            // back
            7, 6, 5,
            5, 4, 7,
            // left
            4, 0, 3,
            3, 7, 4,
            // bottom
            4, 5, 1,
            1, 0, 4,
            // top
            3, 2, 6,
            6, 7, 3,
        ];

        let resources = Resources::from_relative_exe_path(Path::new("res")).unwrap();
        let mut camera = PerspectiveCamera::at_pos(Vector3::new(0.0, 0.0,  5.0));
        camera.set_pos(Vector3::new(0f32, 2f32, 0f32));
        camera.look_at(Vector3::new(0f32, 0f32, -4f32));

        let model = Matrix4::from_translation(Vector3::new(0.0, 0.0, -4.0));
        let view = camera.view_matrix();
        let proj = camera.proj_matrix();
        let mvp = proj * view * model;

        let mut shader_program = ShaderProgram::from_res(&self.gl, &resources, "basic").unwrap();
        shader_program.enable();

        shader_program.set_uniform_mat4f("u_MVP", &mvp);

        let va = VertexArray::new(&self.gl);
        let vb = VertexBuffer::new(&self.gl, cube_vertices.as_ptr() as *const GLvoid, 5 * 8 * size_of::<f32>() as isize);

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(3);
        buffer_layout.push_f32(2);
        va.add_buffer(&vb, &buffer_layout);

        let ib = IndexBuffer::new(&self.gl, cube_indices.as_ptr(), 36);

        let texture = Texture::from_resource(&self.gl, &resources, "textures/grass.png");
        texture.bind(None);
        shader_program.set_uniform_1i("u_Texture", 0);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader_program.disable();

        let renderer = Renderer::new(&self.gl);

        while !self.window.should_close() {
            let time = f32::from_f64(self.glfw.get_time()).unwrap();

            let time_step = TimeStep(time - self.last_frame_time);
            self.last_frame_time = time;

            // Render here
            renderer.clear();

            shader_program.enable();
            let view = camera.view_matrix();
            let proj = camera.proj_matrix();
            let mvp = proj * view * model;
            shader_program.set_uniform_mat4f("u_MVP", &mvp);
            shader_program.disable();

            renderer.draw(&va, &ib, &mut shader_program);

            // Swap front and back buffers
            self.window.swap_buffers();

            // Poll for and process events
            self.glfw.poll_events();
            input::handleMouseInput(&mut self.window, &mut camera);
            input::handleKeyInput(time_step, &self.window, &mut camera);

            for (_, event) in glfw::flush_messages(&self.events) {

                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.window.set_should_close(true);
                    return;
                }

                /*match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
                        camera.set_offset(Vector3::new(0.0, 0.0, -0.2));
                    },
                    glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                        camera.set_offset(Vector3::new(0.0, 0.0, -0.2));
                    },
                    glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                        camera.set_offset(Vector3::new(-0.2, 0.0, 0.0));
                    },
                    glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
                        camera.set_offset(Vector3::new(-0.2, 0.0, 0.0));
                    },
                    glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                        camera.set_offset(Vector3::new(0.0, 0.0, 0.2));
                    },
                    glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
                        camera.set_offset(Vector3::new(0.0, 0.0, 0.2));
                    },
                    glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                        camera.set_offset(Vector3::new(0.2, 0.0, 0.0))
                    },
                    glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
                        camera.set_offset(Vector3::new(0.2, 0.0, 0.0))
                    },
                    _ => {},
                }*/
            }
        }
    }
}

// unsafe fn gl_clear_error(gl: &Gl) {
//     while gl.GetError() != gl::NO_ERROR {}
// }
//
// unsafe fn gl_check_error(gl: &Gl) -> bool {
//     let mut error;
//     loop {
//         error = gl.GetError();
//         if error != gl::NO_ERROR {
//             println!("[OpenGL Error] {}", error);
//             return false;
//         } else {
//             return true;
//         }
//     }
//     return false;
// }

/// The entry function of this binary
fn main() {
    let mut rustcraft = Rustcraft::new();
    rustcraft.run();
}