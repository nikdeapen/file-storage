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
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file as a `Vec` if it exists.
    ///
    /// Returns `Ok(Some(file_content))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_as_vec_if_exists(&self) -> Result<Option<Vec<u8>>, Error> {
        let mut target: Vec<u8> = Vec::default();
        if let Some(content_len) = self.read_to_vec_if_exists(&mut target)? {
            debug_assert_eq!(content_len, target.len());
            Ok(Some(target))
        } else {
            Ok(None)
        }
    }
}
