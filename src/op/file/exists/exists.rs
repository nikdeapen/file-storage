use std::io::ErrorKind::NotFound;

use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if self.is_local_path() {
            self.exists_local()
        } else {
            Err(Error::new(self, Exists, UnknownFileSystem))
        }
    }
}

impl FilePath {
    //! Exists - Local

    fn exists_local(&self) -> Result<bool, Error> {
        debug_assert!(self.is_local_path());

        match std::fs::metadata(self) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Ok(true)
                } else {
                    let message: &str = if metadata.is_dir() {
                        "file path is a folder on the local file system"
                    } else if metadata.is_symlink() {
                        "file path is a symlink on the local file system"
                    } else {
                        "file path is an unknown entity on the local file system"
                    };
                    Err(Error::from_message(self, Exists, message))
                }
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_error(self, Exists, error))
                }
            }
        }
    }
}
