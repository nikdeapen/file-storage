use crate::{Error, FilePath};

impl FilePath {
    //! Write Empty If-Not-Exists

    /// Writes an empty file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already existed.
    pub fn write_empty_if_not_exists(&self) -> Result<bool, Error> {
        self.write_data_if_not_exists(&[0u8; 0])
    }
}
