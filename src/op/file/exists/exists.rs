use crate::LocalPath;
use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
