//! Structs improving the way `OpenGL` is used

use std::ops::Deref;

pub use crate::graphics::bindings::types as types;
pub use crate::graphics::bindings as gl;
use crate::graphics::bindings::types::GLenum;
use std::sync::Arc;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;


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
    inner: Arc<gl::Gl>,
}

pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT:GLenum = 0x84FF;

impl Gl {
    /// Instantiate a new instance of the wrapping `Gl` struct using
    /// `gl::Gl::load_with(...)` under the hood.
    pub fn load_with<F>(load_fn: F) -> Gl
        where F: FnMut(&'static str) -> *const gl::types::GLvoid
    {
        Gl {
            inner: Arc::new(gl::Gl::load_with(load_fn))
        }
    }

    /// Checks whether an `OpenGL` extension is supported by the
    /// graphics driver.
    pub fn ext_supported(&self, name: &str) -> bool {
        let ext_name = CString::new(name).unwrap();
        let mut num_ext= 0;
        unsafe {
            self.GetIntegerv(gl::NUM_EXTENSIONS, &mut num_ext);
            for i in 0..num_ext {
                let ext = self.GetStringi(gl::EXTENSIONS, i as u32);
                let curr_ext_name = CStr::from_ptr(ext as *mut c_char).to_owned();
                if ext_name.eq(&curr_ext_name) {
                    return true;
                }
            }
        }
        false
    }
}

unsafe impl Send for Gl {}
unsafe impl Sync for Gl {}

impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &gl::Gl {
        &self.inner
    }
}