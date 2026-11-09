use crate::{Error, FilePath};

impl FilePath {
    //! Write Empty

    /// Writes a new empty file.
    ///
    /// # Note
    /// - It fails with `FileAlreadyExists` if the file already exists. See `write_data`.
    pub fn write_empty(&self) -> Result<(), Error> {
        self.write_data(b"")
    }

    /// Writes a new empty file if the file does not exist.
    ///
    /// # Note
    /// - It is `true` if the file was written. It is `false` if the file already exists.
    /// - See `write_data_if_not_exists`.
    pub fn write_empty_if_not_exists(&self) -> Result<bool, Error> {
        self.write_data_if_not_exists(b"")
    }
}
