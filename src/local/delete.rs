use std::io::ErrorKind::NotFound;

use crate::local::is_local_path;
use crate::Operation::Delete;
use crate::{Error, FilePath};

pub fn delete_if_exists(file: &FilePath) -> Result<bool, Error> {
    debug_assert!(is_local_path(file));

    match std::fs::remove_file(file) {
        Ok(()) => Ok(true),
        Err(error) => {
            if error.kind() == NotFound {
                Ok(false)
            } else {
                Err(Error::from_error(file, Delete, error))
            }
        }
    }
}
