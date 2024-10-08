use std::io::ErrorKind::NotFound;

use crate::Operation::Delete;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Delete If Exists

    /// Deletes the file if it exists.
    ///
    /// Returns `Ok(true)` if the file existed and was deleted.
    /// Returns `Ok(false)` if the file did not exist.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if self.is_local_path() {
            self.delete_if_exists_local()
        } else {
            Err(Error::new(self, Delete, UnknownFileSystem))
        }
    }
}

impl FilePath {
    //! Delete If Exists - Local

    pub fn delete_if_exists_local(&self) -> Result<bool, Error> {
        match std::fs::remove_file(self.as_str()) {
            Ok(()) => Ok(true),
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_error(self, Delete, error))
                }
            }
        }
    }
}
