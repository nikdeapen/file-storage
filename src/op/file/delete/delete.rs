use crate::Operation::Delete;
use crate::{Error, FilePath};
use crate::{LocalPath, Reason};

impl FilePath {
    //! Delete

    /// Deletes the file.
    ///
    /// Returns `Ok(())` if the file was deleted or if the file did not exist.
    pub fn delete(&self) -> Result<(), Error> {
        if self.is_local_path() {
            return self.delete_if_exists().map(|_| ());
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }

    /// Deletes the file if it exists.
    ///
    /// Returns `Ok(true)` if the file existed and was deleted.
    /// Returns `Ok(false)` if the file did not exist.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.delete_if_exists();
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }
}
