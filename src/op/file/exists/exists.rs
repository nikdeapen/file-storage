use std::io::ErrorKind::NotFound;

use crate::Operation::Exists;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Exists

    /// Checks if the file exists.
    pub fn exists(&self) -> Result<bool, Error> {
        if self.is_local_path() {
            return self.exists_local();
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::r2::R2Path::parse(self.path()) {
            return path.exists();
        }

        Err(Error::new(self.clone(), Exists, UnknownFileSystem))
    }
}

impl FilePath {
    //! Local

    fn exists_local(&self) -> Result<bool, Error> {
        debug_assert!(self.is_local_path());

        match std::fs::metadata(self) {
            Ok(metadata) => {
                if metadata.is_file() {
                    Ok(true)
                } else {
                    let message: &str = if metadata.is_dir() {
                        "the file path is a folder on the local file system"
                    } else if metadata.is_symlink() {
                        "the file path is a symlink on the local file system"
                    } else {
                        "the file path is an unknown entity on the local file system"
                    };
                    Err(Error::from_message(self.clone(), Exists, message))
                }
            }
            Err(error) => {
                if error.kind() == NotFound {
                    Ok(false)
                } else {
                    Err(Error::from_cause(self.clone(), Exists, error))
                }
            }
        }
    }
}
