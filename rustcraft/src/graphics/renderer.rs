use crate::graphics::buffer::{VertexArray, IndexBuffer};
use crate::graphics::shader::{ShaderProgram};
use crate::gl;
use crate::graphics::gl::Gl;

/// A `Renderer` somehow links the whole
/// graphics context together. It combines
/// the given buffers, vertex arrays, shaders and cameras
/// to draw a scene on the screen.
pub struct Renderer {
    /// An `OpenGL` instance
    gl: Gl,
}

impl Renderer {
    /// Creates a new `Renderer`
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGL` instance
    pub fn new(gl: &Gl) -> Self {
        Renderer {
            gl: gl.clone(),
        }
    }
    /// Clears the `OpenGL` rendered context
    pub fn clear(&self) {
        unsafe {
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    /// Draws the given buffers vertex arrysa, shaders and cameras
    ///
    /// # Arguments
    ///
    /// * `va` - A vertex array
    /// * `ib` - An index buffer
    /// * `shader_program` - A shader program
    pub fn draw(&self, va: &VertexArray, ib: &IndexBuffer, shader_program: &mut ShaderProgram) {
        shader_program.enable();
        va.bind();
        ib.bind();

        unsafe {
            self.gl.DrawElements(gl::TRIANGLES, ib.index_count() as i32, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}