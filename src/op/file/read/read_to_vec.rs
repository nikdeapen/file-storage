use crate::system::LocalPath;
use crate::Operation::Read;
use crate::Reason::{FileNotFound, UnknownFileSystem};
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
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file to the `target` `Vec` if it exists.
    ///
    /// Returns `Ok(Some(file_content_len))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.read_to_vec_if_exists(target);
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.read_to_vec_if_exists(target);
        }

        Err(Error::new(self.clone(), Read, UnknownFileSystem))
    }
}
