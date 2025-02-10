use crate::system::LocalPath;
use crate::Operation::Delete;
use crate::{Error, FilePath, Reason};

impl FilePath {
    //! Delete If Exists

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
            return Err(Error::new(
                self.clone(),
                Delete,
                Reason::OperationNotSupported,
            ));
        }

        Err(Error::new(self.clone(), Delete, Reason::UnknownFileSystem))
    }
}
