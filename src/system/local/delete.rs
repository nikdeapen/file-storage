use std::io::ErrorKind::NotFound;

use crate::system::LocalPath;
use crate::Error;
use crate::Operation::Delete;

impl<'a> LocalPath<'a> {
    //! Delete

    /// See `FilePath::delete_if_exists`.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
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
