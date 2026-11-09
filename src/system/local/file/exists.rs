use crate::system::local::is_not_found;
use crate::system::LocalPath;
use crate::Operation::Exists;
use crate::{Error, Operation};

impl<'a> LocalPath<'a> {
    //! Exists

    /// See `FilePath::exists`.
    pub(crate) fn exists(self) -> Result<bool, Error> {
        self.has_file(Exists)
    }

    /// Checks if a file is at the path.
    ///
    /// # Note
    /// - It is `false` if nothing or a folder is at the path.
    /// - It is an error if something else is at the path, such as a FIFO or a device.
    pub(in crate::system::local) fn has_file(self, operation: Operation) -> Result<bool, Error> {
        match std::fs::metadata(self.std_path()) {
            Ok(metadata) if metadata.is_file() => Ok(true),
            Ok(metadata) if metadata.is_dir() => Ok(false),
            Ok(_metadata) => {
                let message: &str = "the file path is an unknown entity on the local file system";
                Err(Error::from_message(self.path.clone(), operation, message))
            }
            Err(error) if is_not_found(&error) => Ok(false),
            Err(error) => Err(Error::from_source(self.path.clone(), operation, error)),
        }
    }
}
