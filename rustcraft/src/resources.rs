use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ResourceError {
    FailedToGetExePath,
}

pub struct Resources {
    /// The root path of the resource directory
    root_path: PathBuf,
}

impl Resources {
    /// Creates a new `Resources` instance.
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, ResourceError> {
        unimplemented!()
    }
}