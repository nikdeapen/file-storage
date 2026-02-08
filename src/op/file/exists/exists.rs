use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};
use crate::{LocalPath, R2Path};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.exists();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::from(self.path()) {
            return path.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
