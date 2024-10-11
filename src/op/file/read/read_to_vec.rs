use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read to Vec

    /// Reads the file to the `target` `Vec`.
    ///
    /// Returns `Ok(file_content_len)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_to_vec(&self, target: &mut Vec<u8>) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_vec_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self, Read, FileNotFound))
        }
    }
}
