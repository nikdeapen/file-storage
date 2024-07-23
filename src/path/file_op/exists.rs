use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    ///
    /// Returns `true` if the file exists and `false` if the files was not found.
    pub fn exists(&self) -> Result<bool, Error> {
        if crate::local::is_local_path(self) {
            return crate::local::exists(self);
        }

        Err(Error::new(self, Exists, UnknownFileSystem))
    }
}
