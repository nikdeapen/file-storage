use std::io::ErrorKind::NotFound;

use crate::path::LocalPath;
use crate::Error;
use crate::Operation::Exists;

impl<'a> LocalPath<'a> {
    //! Exists

    /// See `FilePath::exists`.
    pub fn exists(&self) -> Result<bool, Error> {
        debug_assert!(self.path.is_file());

        match std::fs::metadata(self.path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Ok(true)
                } else {
                    let message: &str = if metadata.is_dir() {
                        "the file path is a folder on the local file system"
                    } else if metadata.is_symlink() {
                        "the file path is a symlink on the local file system"
                    } else {
                        "the file path is an unknown entity on the local file system"
                    };
                    Err(Error::from_message(self.path.clone(), Exists, message))
                }
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_cause(self.path.clone(), Exists, error))
                }
            }
        }
    }
}