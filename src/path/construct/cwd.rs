use crate::{Error, FolderPath, StoragePath};
use std::io;
use std::path::PathBuf;

impl FolderPath {
    //! Current Working Directory

    /// Gets the current working directory.
    pub fn current_working_directory() -> Result<Self, io::Error> {
        let path: PathBuf = std::env::current_dir()?;

        if let Some(path) = path.to_str() {
            Ok(StoragePath::parse(path)?.make_folder())
        } else {
            Err(Error::path_not_utf8(path.as_path()))
        }
    }
}
