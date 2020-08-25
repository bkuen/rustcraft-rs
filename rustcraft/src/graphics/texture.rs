//! Types to represent textures

use crate::graphics::gl::{gl, Gl};
use crate::resources::Resources;
use image::GenericImageView;
use std::os::raw::c_void;
use std::path::PathBuf;

/// Texture
///
/// A `Texture` is used to represent image data
/// in our `OpenGL` context. This data gets
/// processed in the shader.
pub struct Texture {
    /// The id of the texture
    id: u32,
    /// An `OpenGL` instance
    gl: Gl,
    /// The path of the texture file relative to
    /// the textures (resource) directory
    file_path: PathBuf,
    /// The width of the file
    width: u32,
    /// The height of the file
    height: u32,
    /// The bits per pixel
    bpp: u16,
    /// The local buffer of the image
    local_buffer: Vec<u8>,
}

impl Texture {
    /// Creates a new `Texture` from the given `Resources` and its file path
    ///
    /// # Arguments
    ///
    /// * `gl` - An `OpenGL` instance
    /// * `res` - A `Resource` instance
    /// * `file_path` - The file location relative to the
    /// resources root directory.
    pub fn from_resource(gl: &Gl, res: &Resources, file_path: &str) -> Self {
        // Load image from resources
        let mut image = res.load_image(file_path).unwrap();

        // Flip image vertically for `OpenGL` use
        image = image.flipv();

        // Setup `OpenGL`
        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
        }

        // Return a `Texture` instance
        let texture = Self {
            id,
            gl: gl.clone(),
            file_path: PathBuf::from(file_path),
            width: image.width(),
            height: image.height(),
            bpp: image.color().bits_per_pixel(),
            local_buffer: image.into_rgba().into_raw(),
        };

        // Setup `OpenGL` texture parameters and image data
        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, id);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl.TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                texture.width() as i32,
                texture.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture.local_buffer.as_ptr() as *const c_void,
            );
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        texture
    }

    /// Binds the texture in the current `OpenGL` context
    ///
    /// # Arguments
    ///
    /// * `slot_op` - A optional slot the texture should bound to,
    /// default: 0
    pub fn bind(&self, slot_op: Option<u32>) {
        let slot = slot_op.unwrap_or(0);
        unsafe {
            self.gl.ActiveTexture(gl::TEXTURE0 + slot);
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Unbinds the texture from the current `OpenGL` context
    pub fn unbind(&self) {
        unsafe { self.gl.BindTexture(gl::TEXTURE_2D, 0); }
    }

    /// Returns the width of the texture
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the texture
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the bits per pixel of the texture
    pub fn bpp(&self) -> u16 {
        self.bpp
    }

    /// Returns the file path of the texture
    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteTextures(1, &self.id); }
    }
}