//! Types and traits to represent a `GLSL` shader and
//! a shader program.

use crate::graphics::gl::{Gl, gl, types::*};

use std::ffi::{CStr, CString};

/// ShaderType
///
/// A shader could be either one of these:
/// * `Vertex`
/// * `Fragment`
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    /// Returns the `OpenGL` equivalent type
    #[allow(unreachable_patterns)]
    fn opengl_type(&self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            _ => unreachable!()
        }
    }
}

/// Shader
///
/// `OpenGL` requires at least a `VertexShader` and
/// a `FragmentShader` to render entities. These both
/// shaders could be initialized both with this shader
/// implementation and could be bound together via a
/// `ShaderProgram`.
pub struct Shader {
    /// The shader id
    id: GLuint,
    /// A clone of an gl instance
    gl: Gl,
}

impl Shader {
    /// Creates a new `Shader` from a given source
    /// if the source is valid.
    /// Otherwise, it will return the error message.
    ///
    /// # Arguments
    ///
    /// * `gl` - A reference to an `OpenGL` instance
    /// * `source` - A `&CStr` containing the source
    /// code of the shader
    /// * `shader_type` - The type of the shader
    pub fn from_source(gl: &Gl, source: &CStr, shader_type: ShaderType) -> Result<Shader, String> {
        let id = shader_from_source(&gl, source, shader_type.opengl_type())?;
        Ok(Shader {
            id,
            gl: gl.clone()
        })
    }

    /// Creates a new `Shader` from a given vertex shader
    /// source if the source is valid.
    /// Otherwise, it will return the error message.
    ///
    /// # Arguments
    ///
    /// * `gl` - A reference to an `OpenGL` instance
    /// * `source` - A `&CStr` containing the source
    /// code of the vertex shader
    pub fn from_vert_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
        Ok(Shader::from_source(gl, source, ShaderType::Vertex)?)
    }

    /// Creates a new `Shader` from a given fragment shader
    /// source if the source is valid.
    /// Otherwise, it will return the error message.
    ///
    /// # Arguments
    ///
    /// * `gl` - A reference to an `OpenGL` instance
    /// * `source` - A `&CStr` containing the source
    /// code of the fragment shader
    pub fn from_frag_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
        Ok(Shader::from_source(gl, source, ShaderType::Fragment)?)
    }

    /// Returns the shader id
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

/// ShaderProgram
///
/// A `ShaderProgram` is used to link multiple
/// shaders together. Most often, this is just
/// a combination of a vertex and a fragment
/// shader.
pub struct ShaderProgram {
    /// The program id
    id: GLuint,
    /// An `OpenGL` instance
    gl: Gl,
}

impl ShaderProgram {
    /// Creates a shader program and links the given
    /// shaders into it.
    /// If an error occurs, it will return the error
    /// message.
    pub fn from_shaders(gl: &Gl, shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id()); }
        }

        unsafe { gl.LinkProgram(id); }

        let mut success: GLint = 1;
        unsafe {
            gl.GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut GLchar
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe { gl.DetachShader(id, shader.id()); }
        }

        Ok(ShaderProgram {
            id,
            gl: gl.clone()
        })
    }

    /// Enables the shader program
    pub fn enable(&self) {
        unsafe { self.gl.UseProgram(self.id); }
    }

    /// Disables the shader program
    pub fn disable(&self) {
        unsafe { self.gl.UseProgram(0); }
    }

    /// Returns the id of the program
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id); }
    }
}

/// Creates a whitespace `CString` with the given length
///
/// # Arguments
///
/// * `len` - The length of the new whitespace `CString`
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

/// Loads a shader from its source.
/// Returns the shader id if the source is valid.
/// Otherwise it will return the `OpenGL` error
/// message.
///
/// # Arguments
///
/// * `gl` - A reference to an `OpenGL` instance
/// * `source` - A `&CStr` containing the source code
/// of the shader
/// * `kind` - One of the `OpenGL` shader types
fn shader_from_source(gl: &Gl, source: &CStr, kind: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl.CreateShader(kind) };

    unsafe {
        gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl.CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe {
        gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: GLint = 0;
        unsafe {
            gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl.GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut GLchar
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}