use crate::{Error, FilePath};

impl FilePath {
    //! Write Empty

    /// Writes an empty file.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_empty(&self) -> Result<(), Error> {
        self.write_data(&[0u8; 0])
    }
}
