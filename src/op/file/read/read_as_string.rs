use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read as String

    /// Reads the file as a `String` if it exists.
    ///
    /// Returns `Ok(string)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(s) = self.read_as_string_if_exists()? {
            Ok(s)
        } else {
            Err(Error::new(self, Read, FileNotFound))
        }
    }
}
