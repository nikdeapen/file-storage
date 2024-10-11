use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read as Vec

    /// Reads the file as a `Vec`.
    ///
    /// Returns `Ok(file_content)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_as_vec(&self) -> Result<Vec<u8>, Error> {
        if let Some(vec) = self.read_as_vec_if_exists()? {
            Ok(vec)
        } else {
            Err(Error::new(self, Read, FileNotFound))
        }
    }
}
