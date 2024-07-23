use crate::Operation::Delete;
use crate::Reason::{FileNotFound, UnknownFileSystem};
use crate::{Error, FilePath};

impl FilePath {
    //! Delete

    /// Deletes the file.
    ///
    /// # Errors
    /// `FileNotFound` - if the file did not exist.
    pub fn delete(&self) -> Result<(), Error> {
        let deleted: bool = self.delete_if_exists()?;
        if deleted {
            Ok(())
        } else {
            Err(Error::new(self, Delete, FileNotFound))
        }
    }

    /// Deletes the file if it exists.
    ///
    /// Returns `true` if the file was deleted and `false` if the file did not exist.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if crate::local::is_local_path(self) {
            return crate::local::delete_if_exists(self);
        }

        Err(Error::new(self, Delete, UnknownFileSystem))
    }
}
