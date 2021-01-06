//! Types to represent textures

use crate::graphics::gl::{gl, Gl};
use crate::resources::Resources;
use image::GenericImageView;
use std::os::raw::c_void;
use std::path::PathBuf;
use std::ops::{Deref, DerefMut};
use cgmath::Vector2;

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
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
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

/// SubTexture
///
/// A `SubTexture` represents one sprite of a texture atlas
pub struct SubTexture<'a> {
    /// The texture atlas this sub texture is referring
    tex_atlas: &'a TextureAtlas,
    /// The texture coordinates of this sub texture
    tex_coords: [f32; 8]
}

impl<'a> SubTexture<'a> {

    /// Creates a new sub texture from min and max coordinates
    ///
    /// # Arguments
    ///
    /// * `tex_atlas` - A reference to a texture atlas
    /// * `min` - The min coordinate of the sub texture
    /// * `max` - The max coordinate of the sub texture
    fn new(tex_atlas: &'a TextureAtlas, min: Vector2<f32>, max: Vector2<f32>) -> Self {
        let tex_coords= [
            min.x, min.y,
            max.x, min.y,
            max.x, max.y,
            min.x, max.y,
        ];
        Self {
            tex_atlas,
            tex_coords,
        }
    }

    /// Returns the texture coords as a `[f32; 8]`
    pub fn coords(&self) -> &[f32; 8] {
       &self.tex_coords
    }
}

/// TextureAtlas
///
/// A `TextureAtlas` combines multiple textures in just one file.
/// Therefore, only one texture needs to be load with `OpenGL`.
/// With this in place, the texture coordinates for each sprite
/// could be calculated using the `total width/height` and `sprite
/// width/length`
pub struct TextureAtlas {
    /// The underlying texture
    texture: Texture,
    /// The size of each sprite in the texture atlas
    sprite_size: Vector2<f32>,
}

impl Deref for TextureAtlas {
    type Target = Texture;

    fn deref(&self) -> &Self::Target {
        &self.texture
    }
}

impl DerefMut for TextureAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.texture
    }
}

impl TextureAtlas {
    /// Creates a new texture atlas from a given texture
    ///
    /// # Arguments
    ///
    /// * `texture` - The underlying texture
    /// * `sprite_size` - The size of each sprite
    pub fn from_texture(texture: Texture, sprite_size: Vector2<f32>) -> Self {
        return Self {
            texture,
            sprite_size,
        }
    }

    /// Returns the sub texture within the given coords
    ///
    /// # Argument
    ///
    /// * `coords` - The relative coordinates to a sub texture of the atlas
    pub fn sub_texture(&self, coords: Vector2<f32>) -> SubTexture {
        let min: Vector2<f32> = Vector2::new(
            (coords.x * self.sprite_size.x) / self.width as f32,
            (coords.y * self.sprite_size.y) / self.height as f32,
        );
        let max: Vector2<f32> = Vector2::new(
            ((coords.x + 1.0) * self.sprite_size.x) / self.width as f32,
            ((coords.y + 1.0) * self.sprite_size.y) / self.height as f32,
        );
        SubTexture::new(&self, min, max)
    }
}