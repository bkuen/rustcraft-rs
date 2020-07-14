//! Types and traits to represent a `GLSL` shader.

use crate::gl::types::*;

/// ShaderType
///
/// A shader could be either one of these:
/// + `Vertex`
/// + `Fragment`
pub enum ShaderType {
    Vertex,
    Fragment
}

/// Shader
///
/// `OpenGL` requires at least a `VertexShader` and
/// a `FragmentShader` to render entities. These both
/// shaders are stored in the `Shader` struct and linked
/// via an `OpenGL Program`.
pub struct Shader {
    /// The shader program id
    program_id: Option<GLuint>,
    /// The vertex shader id
    vertex_shader_id: Option<GLuint>,
    /// The fragment shader id
    fragment_shader_id: Option<GLuint>,
}

impl Shader {
    /// Instantiate a new `Shader` object.
    pub fn new() -> Self {
        Shader {
            fragment_shader_id: None,
            program_id: None,
            vertex_shader_id: None,
        }
    }

    /// Load a shader from a `GLSL` file
    /// by passing the file.
    pub fn load_shader_from_file(&mut self, file: &str, shader_type: ShaderType) {
        unimplemented!()
    }

    /// Load a shader from a `GLSL` file
    /// by passing its location
    pub fn load_shader_from_path(&mut self, path: &str, shader_type: ShaderType) {
        unimplemented!()
    }

    /// Load a shader program
    pub fn load_program(&mut self) {
        unimplemented!()
    }

    /// Enable shader program
    pub fn enable(&self) {
        unimplemented!()
    }

    /// Disable shader program
    pub fn disable(&self) {
        unimplemented!()
    }

    /// Returns the shader program id
    pub fn program(&self) -> Option<GLuint> {
        self.program_id
    }
}