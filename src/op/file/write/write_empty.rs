use crate::{Error, FilePath};

impl FilePath {
    //! Write Empty

    /// Writes an empty file.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_empty(&self) -> Result<(), Error> {
        self.write_data([0u8; 0])
    }

    /// Writes an empty file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    pub fn write_empty_if_not_exists(&self) -> Result<bool, Error> {
        self.write_data_if_not_exists([0u8; 0])
    }
}
