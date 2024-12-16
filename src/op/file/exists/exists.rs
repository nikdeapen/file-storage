use crate::path::LocalPath;
use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if let Some(local) = LocalPath::parse(self.path()) {
            return local.exists();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::r2::R2Path::parse(self.path()) {
            return path.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}
