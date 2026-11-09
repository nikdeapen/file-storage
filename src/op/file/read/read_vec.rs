use crate::Operation::Read;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::{FileNotFound, UnknownFileSystem};
use crate::{Error, FilePath, LocalPath};

impl FilePath {
    //! Read Vec

    /// Reads the file as a `Vec`.
    ///
    /// # Note
    /// - A missing file fails with `FileNotFound`. See `read_if_exists` for what counts as missing.
    pub fn read_as_vec(&self) -> Result<Vec<u8>, Error> {
        if let Some(vec) = self.read_as_vec_if_exists()? {
            Ok(vec)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file as a `Vec` if it exists.
    ///
    /// # Note
    /// - It is `None` if the file does not exist. See `read_if_exists` for what counts as missing.
    pub fn read_as_vec_if_exists(&self) -> Result<Option<Vec<u8>>, Error> {
        let mut target: Vec<u8> = Vec::default();
        if let Some(content_len) = self.read_to_vec_if_exists(&mut target)? {
            debug_assert_eq!(content_len, target.len());
            Ok(Some(target))
        } else {
            Ok(None)
        }
    }

    /// Reads the file to the end of the `target` `Vec`.
    ///
    /// # Note
    /// - It returns the number of bytes read.
    /// - A missing file fails with `FileNotFound`. See `read_if_exists` for what counts as missing.
    /// - On error the `target` is left as it was.
    pub fn read_to_vec(&self, target: &mut Vec<u8>) -> Result<usize, Error> {
        if let Some(file_content_len) = self.read_to_vec_if_exists(target)? {
            Ok(file_content_len)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file to the end of the `target` `Vec` if it exists.
    ///
    /// # Note
    /// - It returns the number of bytes read. It is `None` if the file does not exist.
    /// - See `read_if_exists` for what counts as missing.
    /// - On error the `target` is left as it was.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        let original_len: usize = target.len();
        let result: Result<Option<usize>, Error> = self.read_to_vec_from_backend(target);
        if result.is_err() {
            target.truncate(original_len);
        }
        result
    }

    /// Reads the file to the `target` `Vec` from its backend.
    fn read_to_vec_from_backend(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.read_to_vec_if_exists(target);
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.read_to_vec_if_exists(target);
        }

        Err(Error::new(self.clone(), Read, UnknownFileSystem))
    }
}
