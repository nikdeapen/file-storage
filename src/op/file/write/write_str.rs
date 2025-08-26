use crate::{Error, FilePath};

impl FilePath {
    //! Write String

    /// Writes the `string` to the file.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_str<S>(&self, string: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        self.write_slice(string.as_ref())
    }

    /// Writes the `string` to the file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    pub fn write_str_if_not_exists<S>(&self, string: S) -> Result<bool, Error>
    where
        S: AsRef<str>,
    {
        self.write_slice_if_not_exists(string.as_ref().as_bytes())
    }
}
