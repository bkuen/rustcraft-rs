//! Types and traits representing graphics buffers
//! like `VertexBuffer`s, `IndexBuffer`s and
//! `VertexArray`s

use crate::graphics::gl::{Gl, gl, types::*};
use std::mem::size_of;
use std::any::type_name;
use std::os::raw::c_uchar;
use std::slice::Iter;

/// VertexBuffer
///
/// A `VertexBuffer` is used to store
/// any kinds of vertices.
pub struct VertexBuffer {
    /// The id of the vertex buffer
    id: GLuint,
    /// An `OpenGL` instance
    gl: Gl,
}

impl VertexBuffer {
    /// Creates a new vertex buffer
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGL` instance
    /// * `data` - A pointer to the data
    /// * `size` - The size of the data
    pub fn new(gl: &Gl, data: *const GLvoid, size: isize) -> Self {
        let mut buffer: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut buffer);
            gl.BindBuffer(gl::ARRAY_BUFFER, buffer);
            gl.BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);
        }

        VertexBuffer {
            gl: gl.clone(),
            id: buffer,
        }
    }

    /// Binds the buffer
    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, self.id); }
    }

    /// Unbinds the buffer
    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(gl::ARRAY_BUFFER, 0); }
    }

    /// Returns the id of the `VertexBuffer`
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &self.id); }
    }
}

/// IndexBuffer
///
/// A `IndexBuffer` is used to store
/// a bunch of indices for different
/// vertices.
/// At the moment, just 32 bit integers
/// are allowed.
pub struct IndexBuffer {
    /// The id of the vertex buffer
    id: GLuint,
    /// An `OpenGL` instance
    gl: Gl,
    /// The index count
    index_count: usize,
}

impl IndexBuffer {
    /// Creates a new `IndexBuffer` from the
    /// given indices and stores its length.
    ///
    /// # Arguments
    ///
    /// * `gl` - A reference to an `OpenGL` instance
    /// * `indices` - A pointer to the data
    /// * `index_count` - The index count of the data
    pub fn new(gl: &Gl, indices: *const u32, index_count: usize) -> Self {
        let mut buffer: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut buffer);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_count * size_of::<u32>()) as isize,
                indices as *const GLvoid,
                gl::STATIC_DRAW
            );
        }

        IndexBuffer {
            gl: gl.clone(),
            id: buffer,
            index_count
        }
    }

    /// Binds the buffer
    pub fn bind(&self) {
        unsafe { self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id); }
    }

    /// Unbinds the buffer
    pub fn unbind(&self) {
        unsafe { self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); }
    }

    /// Returns the id of the `IndexBuffer`
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// Returns the index count of the `IndexBuffer`
    pub fn index_count(&self) -> usize {
        self.index_count
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteBuffers(1, &self.id); }
    }
}

/// VertexBufferElement
///
struct VertexBufferElement {
    count: i32,
    element_type: u32,
    normalized: u8,
}

impl VertexBufferElement {
    /// Returns the `OpenGL` type as
    /// `GLenum`
    ///
    /// # Arguments
    ///
    /// * `opengl_type` - An `OpenGL` type
    fn size_of_opengl_type(opengl_type: u32) -> i32 {
        match opengl_type {
            gl::FLOAT => 4,
            gl::UNSIGNED_INT => 4,
            gl::UNSIGNED_BYTE => 4,
            _ => panic!("Unsupported type!"),
        }
    }
}

/// VertexBufferLayout
///
pub struct VertexBufferLayout {
    /// A vector of vertex buffer elements
    elements: Vec<VertexBufferElement>,
    /// The stride of the layout
    stride: i32,
}

impl VertexBufferLayout {
    /// Creates a new `VertexBufferLayout`
    pub fn new() -> Self {
        VertexBufferLayout {
            elements: Vec::new(),
            stride: 0,
        }
    }

    /// Pushes a new element to the layout
    ///
    /// This method uses the type name of the give type to
    /// transform the type in an `OpenGL`-known type.
    /// Actually, this isn't best practice and should be replaced
    /// by an own `TypeWrapper` with the `OpenGL` types associated.
    pub fn push<T: ?Sized>(&mut self, count: i32, normalized: u8) {
        let element_type: GLenum;

        match type_name::<T>() {
            "f32" => element_type = gl::FLOAT,
            "u32" => element_type = gl::UNSIGNED_INT,
            "i32" => element_type = gl::INT,
            "os::raw::c_uchar" => element_type = gl::UNSIGNED_BYTE,
            _ => panic!("Unsupported type!"),
        }

        self.stride += VertexBufferElement::size_of_opengl_type(element_type) * count;
        self.elements.push(VertexBufferElement {
            count,
            element_type,
            normalized,
        });
    }

    /// Push a new f32 element to the layout
    pub fn push_f32(&mut self, count: i32) {
        self.push::<f32>(count, gl::FALSE);
    }

    /// Push a new u32 element to the layout
    pub fn push_u32(&mut self, count: i32) {
        self.push::<u32>(count, gl::FALSE);
    }

    /// Push a new i32 element to the layout
    pub fn push_i32(&mut self, count: i32) {
        self.push::<i32>(count, gl::FALSE);
    }

    /// Push a new f32 element to the layout
    pub fn push_uchar(&mut self, count: i32) {
        self.push::<c_uchar>(count, gl::TRUE);
    }

    /// Returns the elements of the layout as
    /// an iterator
    fn elements(&self) -> Iter<'_, VertexBufferElement> {
        self.elements.iter()
    }

    /// Returns the stride of the layout
    fn stride(&self) -> i32 {
        self.stride
    }
}

/// VertexArray
///
/// A vertex array is supposed to tie together a
/// buffer with an actual layout
pub struct VertexArray {
    /// An `OpenGL` instance
    gl: Gl,
    /// The id of the `VertexArray`
    id: GLuint,
    /// The buffer count
    buffer_count: u8,
}

impl VertexArray {
    /// Create a new vertex array
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGL` instance
    pub fn new(gl: &Gl) -> Self {
        let mut vao: GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        VertexArray {
            id: vao,
            gl: gl.clone(),
            buffer_count: 0,
        }
    }

    /// Add a buffer to the vertex array
    pub fn add_buffer(&mut self, vb: &VertexBuffer, layout: &VertexBufferLayout) {
        let mut offset = 0;

        self.bind();
        vb.bind();
        layout.elements().for_each(|element | unsafe {
            let index = self.buffer_count as u32;
            self.gl.EnableVertexAttribArray(index);
            self.gl.VertexAttribPointer(index, element.count, element.element_type, element.normalized, layout.stride(), offset as *const gl::types::GLvoid);
            offset += element.count * VertexBufferElement::size_of_opengl_type(element.element_type);
            self.buffer_count += 1;
        });
    }

    /// Binds the vertex array
    pub fn bind(&self) {
        unsafe { self.gl.BindVertexArray(self.id); }
    }

    /// Unbinds the vertex array
    pub fn unbind(&self) {
        unsafe { self.gl.BindVertexArray(self.id); }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &self.id); }
    }
}