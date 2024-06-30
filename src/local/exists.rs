use std::io::ErrorKind::NotFound;

use crate::local::is_local_path::is_local_path;
use crate::Operation::Exists;
use crate::{Error, FilePath};

pub fn exists(file: &FilePath) -> Result<bool, Error> {
    debug_assert!(is_local_path(file));

    match std::fs::metadata(file) {
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
                Err(Error::from_message(file, Exists, message))
            }
        }
        Err(error) => {
            if error.kind() == NotFound {
                Ok(false)
            } else {
                Err(Error::from_error(file, Exists, error))
            }
        }
    }
}
