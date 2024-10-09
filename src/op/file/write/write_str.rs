use crate::{Error, FilePath};
use crate::Operation::Write;
use crate::Reason::FileAlreadyExists;

impl FilePath {
    //! Write String

    /// Writes the string `s`.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_str<S>(&self, s: S) -> Result<(), Error>
    where
        S: AsRef<str>,
    {
        if self.write_str_if_not_exists(s.as_ref())? {
            Ok(())
        } else {
            Err(Error::new(self, Write, FileAlreadyExists))
        }
    }
}
