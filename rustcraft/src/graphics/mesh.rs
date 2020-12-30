//! Types to represent meshes and models

use crate::graphics::buffer::{VertexArray, VertexBuffer, VertexBufferLayout, IndexBuffer};
use crate::graphics::gl::Gl;
use crate::graphics::bindings::types::GLvoid;
use std::mem::size_of;

/// Mesh
///
/// A mesh holds the vertex positions, texture coords
/// and indices which will be rendered to the screen
pub struct Mesh {
    pub vertex_positions: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            vertex_positions: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
        }
    }
}

/// Model
///
/// A model is built up by a mesh and it is generating the
/// required buffers for an `OpenGL` render call
pub struct Model {
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
    pub fn from_mesh(gl: &Gl, mesh: &Mesh) -> Self {
        let mut va = VertexArray::new(gl);
        let vb_vertex_positions = VertexBuffer::new(gl, mesh.vertex_positions.as_ptr() as *const GLvoid, mesh.vertex_positions.len() as isize * size_of::<f32>() as isize);
        let vb_tex_coords = VertexBuffer::new(gl, mesh.tex_coords.as_ptr() as *const GLvoid, mesh.tex_coords.len() as isize * size_of::<f32>() as isize);

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(3);
        va.add_buffer(&vb_vertex_positions, &buffer_layout);

        let mut buffer_layout = VertexBufferLayout::new();
        buffer_layout.push_f32(2);
        va.add_buffer(&vb_tex_coords, &buffer_layout);

        let ib = IndexBuffer::new(gl, mesh.indices.as_ptr(), mesh.indices.len());

        let buffers = vec![vb_vertex_positions, vb_tex_coords];

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