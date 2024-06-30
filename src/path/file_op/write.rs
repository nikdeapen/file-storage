use std::io::Write;

use crate::op::FileWrite;
use crate::ErrorInfo::{FileAlreadyExists, UnknownFileSystem};
use crate::Operation;
use crate::{Error, FilePath};

impl FilePath {
    //! Write

    /// Writes the file. A new file will be created.
    ///
    /// # Errors
    /// `FileAlreadyExists` - if the file already exists.
    pub fn write(&self) -> Result<FileWrite, Error> {
        if let Some(write) = self.write_if_not_exists()? {
            Ok(write)
        } else {
            Err(Error::new(self, Operation::Write, FileAlreadyExists))
        }
    }

    /// Writes the file if the file does not exist.
    pub fn write_if_not_exists(&self) -> Result<Option<FileWrite>, Error> {
        #[cfg(feature = "local")]
        if crate::local::is_local_path(self) {
            return crate::local::write_if_not_exists(self);
        }

        Err(Error::new(self, Operation::Write, UnknownFileSystem))
    }

    /// Writes an empty file. A new file will be created.
    ///
    /// # Errors
    /// `FileAlreadyExists` - if the file already exists.
    pub fn write_empty_file(&self) -> Result<(), Error> {
        self.write()?
            .commit()
            .map_err(|error| Error::from_error(self, Operation::Write, error))
    }

    /// Writes the data to the file. A new file will be created.
    ///
    /// # Errors
    /// `FileAlreadyExists` - if the file already exists.
    pub fn write_data<D>(&self, data: D) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        let mut write: FileWrite = self.write()?;
        write
            .write_all(data.as_ref())
            .map_err(|error| Error::from_error(self, Operation::Write, error))?;
        write
            .commit()
            .map_err(|error| Error::from_error(self, Operation::Write, error))
    }
}
