use crate::system::LocalPath;
use crate::Operation::Write;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath, FileWrite};

impl FilePath {
    //! Write

    /// Writes an empty file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    pub fn write(&self) -> Result<FileWrite, Error> {
        if let Some(write) = self.write_if_not_exists()? {
            Ok(write)
        } else {
            Err(Error::new(self.path().clone(), Write, FileAlreadyExists))
        }
    }

    /// Opens a write operation to the file.
    ///
    /// Returns `Ok(file_write)`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_if_not_exists(&self) -> Result<Option<FileWrite>, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.write_if_not_exists();
        }

        // todo -- R2

        Err(Error::new(self.path().clone(), Write, UnknownFileSystem))
    }
}
