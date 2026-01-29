use crate::Operation::Read;
use crate::Reason::{FileNotFound, UnknownFileSystem};
use crate::{Error, FilePath, LocalPath};

impl FilePath {
    //! Read Vec

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

    /// Reads the file to the `target` `Vec`.
    ///
    /// Returns `Ok(file_content_len)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_to_vec(&self, target: &mut Vec<u8>) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_vec_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file to the `target` `Vec` if it exists.
    ///
    /// Returns `Ok(Some(file_content_len))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        if let Some(path) = LocalPath::from(self.path()) {
            return path.read_to_vec_if_exists(target);
        }

        Err(Error::new(self.clone(), Read, UnknownFileSystem))
    }
}
