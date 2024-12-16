use crate::Operation::Write;
use crate::Reason::FileAlreadyExists;
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
        if self.write_str_if_not_exists(string.as_ref())? {
            Ok(())
        } else {
            Err(Error::new(self.clone(), Write, FileAlreadyExists))
        }
    }
}
