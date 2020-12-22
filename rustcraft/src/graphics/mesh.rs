//! Types to represent meshes and models

use crate::graphics::buffer::{VertexArray, VertexBuffer, VertexBufferLayout, IndexBuffer};
use crate::graphics::gl::Gl;
use crate::graphics::bindings::types::GLvoid;
use std::mem::size_of;

/// Mesh
///
/// A mesh holds the vertex positions, texture coords
/// and indices which will be rendered to the screen
struct Mesh {
    vertex_positions: Vec<f32>,
    tex_coords: Vec<f32>,
    indices: Vec<u32>,
}

/// Model
///
/// A model is built up by a mesh and it is generating the
/// required buffers for an `OpenGL` render call
struct Model {
    /// An `OpenGL` instance
    gl: Gl,
    /// The vertex array of the model
    va: VertexArray,
    /// The index buffer of the model
    ib: IndexBuffer,
    /// All additional buffers for the model.
    /// At the moment, only vertex buffers are supported to be stored here.
    /// This might change in the future.
    buffers: Vec<VertexBuffer>,
}

impl Model {
    /// Creates a new model from a given mesh
    ///
    /// # Arguments
    ///
    /// * `mesh` - A mesh instance
    fn from_mesh(gl: &Gl, mesh: &Mesh) -> Self {
        let va = VertexArray::new(gl);
        let vb = VertexBuffer::new(gl, mesh.vertex_positions.as_ptr() as *const GLvoid, (mesh.vertex_positions.len() * size_of::<f32>()) as isize);
        let ib = IndexBuffer::new(gl, mesh.indices.as_ptr(), mesh.indices.len());

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(3);
        buffer_layout.push_f32(2);

        va.add_buffer(&vb, &buffer_layout);

        let buffers = vec![vb];

        Self {
            va,
            ib,
            buffers,
            gl: gl.clone(),
        }
    }

    /// Binds the model
    pub fn bind(&self) {
        self.va.bind();
        self.ib.bind();
    }

    /// Unbinds the model
    pub fn unbind(&self) {
        self.va.unbind();
        self.ib.unbind();
    }

    /// Returns the vertex array of the model
    pub fn va(&self) -> &VertexArray {
       &self.va
    }

    /// Returns the index buffer of the model
    pub fn ib(&self) -> &IndexBuffer {
        &self.ib
    }

    /// Returns all additional buffers.
    /// At the moment, only vertex buffers are supported to be stored here.
    /// This might change in the future.
    pub fn buffers(&self) -> &Vec<VertexBuffer> {
        &self.buffers
    }
}