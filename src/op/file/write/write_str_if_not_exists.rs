use crate::{Error, FilePath};

impl FilePath {
    //! Write Data If Not Exists

    /// Writes the string `s` to the file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already existed.
    pub fn write_str_if_not_exists<S>(&self, s: S) -> Result<bool, Error>
    where
        S: AsRef<str>,
    {
        self.write_data_if_not_exists(s.as_ref().as_bytes())
    }
}
