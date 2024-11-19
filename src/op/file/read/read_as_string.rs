use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read as String

    /// Reads the file as a `String`.
    ///
    /// Returns `Ok(file_content)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(file_content) = self.read_as_string_if_exists()? {
            Ok(file_content)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }
}
