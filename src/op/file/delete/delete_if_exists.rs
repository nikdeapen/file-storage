use std::io::ErrorKind::NotFound;

use crate::Operation::Delete;
use crate::{Error, FilePath, Reason};

impl FilePath {
    //! Delete If Exists

    /// Deletes the file if it exists.
    ///
    /// Returns `Ok(true)` if the file existed and was deleted.
    /// Returns `Ok(false)` if the file did not exist.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if self.is_local_path() {
            return self.delete_if_exists_local();
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }
}

impl FilePath {
    //! Local

    /// See `FilePath::delete_if_exists`.
    pub fn delete_if_exists_local(&self) -> Result<bool, Error> {
        match std::fs::remove_file(self.as_str()) {
            Ok(()) => Ok(true),
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_cause(self.clone(), Delete, error))
                }
            }
        }
    }
}
