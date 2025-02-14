use crate::system::LocalPath;
use crate::Operation::Delete;
use crate::Reason::{OperationNotSupported, UnknownFileSystem};
use crate::{Error, FilePath};

impl FilePath {
    //! Delete

    /// Deletes the file.
    ///
    /// Returns `Ok(())` if the file was deleted or if the file did not exist.
    pub fn delete(&self) -> Result<(), Error> {
        if self.is_local_path() {
            return self.delete_if_exists().map(|_| ());
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.delete();
        }

        Err(Error::new(self.clone(), Delete, UnknownFileSystem))
    }

    /// Deletes the file if it exists.
    ///
    /// Returns `Ok(true)` if the file existed and was deleted.
    /// Returns `Ok(false)` if the file did not exist.
    pub fn delete_if_exists(&self) -> Result<bool, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.delete_if_exists();
        }

        #[cfg(feature = "r2")]
        if let Some(_r2) = crate::R2Path::from(self.path()) {
            return Err(Error::new(self.clone(), Delete, OperationNotSupported));
        }

        Err(Error::new(self.clone(), Delete, UnknownFileSystem))
    }
}
