//! Entry point and types/trait representing the
//! application/game.

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key, Glfw, Window, WindowEvent};

pub mod gl;
pub mod graphics;

/// Gl
///
/// This struct is a wrapper around the `Gl` struct
/// from the generated `OpenGL` source. It's used to
/// reduce the amount of bytes from ~10mb to ~8b per
/// copy. With this in place, the `GL` 'instance'
/// could be cloned effectively.
///
/// Internally, a reference counted pointer is used
/// to store the address to the `GL` instance. Moreover,
/// the `Deref` trait is implemented to grant access to
/// the associated types.
#[derive(Clone)]
pub struct Gl {
    inner: Rc<gl::Gl>,
}

impl Gl {
    /// Instantiate a new instance of the wrapping `Gl` struct using
    /// `gl::Gl::load_with(...)` under the hood.
    pub fn load_with<F>(load_fn: F) -> Gl
        where F: FnMut(&'static str) -> *const gl::types::GLvoid
    {
        Gl {
            inner: Rc::new(gl::Gl::load_with(load_fn))
        }
    }
}

impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &gl::Gl {
        &self.inner
    }
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
}

impl Rustcraft {
    /// Initialize a new `Rustcraft` application
    /// by creating an event loop, a window and
    /// an `OpenGL` instance/context.
    pub fn new() -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
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
        while !self.window.should_close() {

            unsafe {
                self.gl.ClearColor(1.0, 0.4, 0.4, 1.0);
                self.gl.Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.swap_buffers();
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events) {
                println!("{:?}", event);
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

/// The entry function of this binary
fn main() {
    let mut rustcraft = Rustcraft::new();
    rustcraft.run();
}