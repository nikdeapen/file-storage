use crate::LocalPath;
use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.exists();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::new(self.path()) {
            return path.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
