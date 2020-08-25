//! Types implementing a simple resources system

use std::ffi;
use std::fs;
use std::io::{self, Read, Error};
use std::path::{Path, PathBuf};
use image::{ImageError, DynamicImage};

#[derive(Debug)]
pub enum ResourceError {
    FailedToGetExePath,
    FileContainsNil,
    Io(io::Error),
    Image(image::ImageError),
}

impl From<io::Error> for ResourceError {
    fn from(error: Error) -> Self {
        ResourceError::Io(error)
    }
}

impl From<image::ImageError> for ResourceError {
    fn from(error: ImageError) -> Self {
        ResourceError::Image(error)
    }
}

pub struct Resources {
    /// The root path of the resource directory
    root_path: PathBuf,
}

impl Resources {
    /// Creates a new `Resources` instance from a given path
    /// relative to the executable. This function might end
    /// in a `ResourceError` if problems with the given path
    /// occur.
    ///
    /// # Arguments
    ///
    /// * `rel_path` - A relative path to a resource directory relative to
    /// the executable
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, ResourceError> {
        let exe_file_name = ::std::env::current_exe()
            .map_err(|_| ResourceError::FailedToGetExePath)?;

        let exe_path = exe_file_name.parent()
            .ok_or(ResourceError::FailedToGetExePath)?;

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    /// Loads a cstring out of an file located in a resource directory.
    /// This function might end in a `ResourceError` if the file could
    /// somehow not be read correctly.
    ///
    /// # Arguments
    ///
    /// * `resource_name` - The resource name the cstring should be read.
    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, ResourceError> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        // allocate buffer of the same size as file
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // check for nil byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(ResourceError::FileContainsNil)
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer)})
    }

    /// Loads a image from a resource directory.
    ///
    /// # Arguments
    ///
    /// * `resource_name` - The resource name the image should be read.
    pub fn load_image(&self, resource_name: &str) -> Result<DynamicImage, ResourceError> {
        let path = resource_name_to_path(&self.root_path, resource_name);
        let image = image::open(path)?;
        Ok(image)
    }
}

/// Helper function which takes a root directory and a path location
/// to create a platform independent path by splitting over all `/` and
/// adding them to the path with the correct separator internally.
///
/// # Arguments
///
/// * `root_dir` - The root (resource) directory
/// * `location` . The (relative) location to the file
fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}