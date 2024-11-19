use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read to String

    /// Reads the file to the `target` `String`.
    ///
    /// Returns `Ok(file_content_len)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_to_string(&self, target: &mut String) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_string_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }
}
