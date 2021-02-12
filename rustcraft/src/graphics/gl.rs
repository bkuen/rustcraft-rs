//! Structs improving the way `OpenGL` is used

use std::rc::Rc;
use std::ops::Deref;

pub use crate::graphics::bindings::types as types;
pub use crate::graphics::bindings as gl;
use std::sync::Arc;

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
}

unsafe impl Send for Gl {}
unsafe impl Sync for Gl {}

impl Deref for Gl {
    type Target = gl::Gl;

    fn deref(&self) -> &gl::Gl {
        &self.inner
    }
}