use crate::system::LocalPath;
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

        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
