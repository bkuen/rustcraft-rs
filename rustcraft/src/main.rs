//! Entry point and types/trait representing the
//! application/game.

use crate::graphics::buffer::{VertexBuffer, IndexBuffer, VertexArray, VertexBufferLayout};
use crate::graphics::gl::{Gl, gl, types::*};
use crate::graphics::shader::{ShaderProgram};

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent, SwapInterval, OpenGlProfileHint};

use std::mem::size_of;
use std::sync::mpsc::Receiver;
use crate::graphics::renderer::Renderer;
use crate::resources::Resources;
use std::path::Path;
use crate::graphics::texture::Texture;

pub mod graphics;
pub mod resources;

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

        let gl = Gl::load_with(|s| window.get_proc_address(s) as *const std::os::raw::c_void);
        Self {
            glfw,
            gl,
            events,
            window,
        }
    }

    /// Create a new `GLFW` window with a title
    fn create_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
        let (mut window, events) = glfw.create_window(1280, 720, "", glfw::WindowMode::Windowed)
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
            self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let positions: [f32; 16] = [
        //  Coords      Texture coords
            -0.5, -0.5, 0.0, 0.0,
             0.5, -0.5, 1.0, 0.0,
             0.5,  0.5, 1.0, 1.0,
            -0.5,  0.5, 0.0, 1.0,
        ];

        let indices: [u32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let resources = Resources::from_relative_exe_path(Path::new("res")).unwrap();
        let mut shader_program = ShaderProgram::from_res(&self.gl, &resources, "basic").unwrap();

        // let mut shader_program = ShaderProgram::from_shaders(&self.gl, &[vs, fs]).unwrap();
        shader_program.enable();
        shader_program.set_uniform_4f("u_Color", 0.3, 0.8, 0.6, 1.0);

        let va = VertexArray::new(&self.gl);
        let vb = VertexBuffer::new(&self.gl, positions.as_ptr() as *const GLvoid, 4 * 4 * size_of::<f32>() as isize);

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(2);
        buffer_layout.push_f32(2);
        va.add_buffer(&vb, &buffer_layout);

        let ib = IndexBuffer::new(&self.gl, indices.as_ptr(), 6);

        let texture = Texture::from_resource(&self.gl, &resources, "textures/dirt_block.jpg");
        texture.bind(None);
        shader_program.set_uniform_1i("u_Texture", 0);

        va.unbind();
        vb.unbind();
        ib.unbind();
        shader_program.disable();

        let renderer = Renderer::new(&self.gl);

        let mut g = 0.0;
        let mut inc = 0.05;
        while !self.window.should_close() {
            // Render here
            renderer.clear();

            shader_program.enable();
            shader_program.set_uniform_4f("u_Color", 0.3, g, 0.6, 1.0);

            renderer.draw(&va, &ib, &mut shader_program);

            if g > 1.0 {
                inc = -0.05;
            } else if g < 0.0 {
                inc = 0.05;
            }
            g += inc;

            // Swap front and back buffers
            self.window.swap_buffers();

            // Poll for and process events
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
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