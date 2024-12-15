use std::io::ErrorKind::NotFound;

use crate::path::LocalPath;
use crate::Error;
use crate::Operation::Delete;

impl<'a> LocalPath<'a> {
    //! Delete

    /// See `FilePath::delete_if_exists`.
    pub fn delete_if_exists_local(&self) -> Result<bool, Error> {
        debug_assert!(self.path.is_file());

        match std::fs::remove_file(self.path.as_str()) {
            Ok(()) => Ok(true),
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_cause(self.path.clone(), Delete, error))
                }
            }
        }
    }
}
