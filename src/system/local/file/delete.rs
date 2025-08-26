use crate::system::LocalPath;
use crate::Error;
use crate::Operation::Delete;
use std::io::ErrorKind::NotFound;

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
                    Err(Error::from_source(self.path.clone(), Delete, error))
                }
            }
        }
    }
}
