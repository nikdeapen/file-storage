use crate::path::LocalPath;
use crate::Operation::Read;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Read to Vec If Exists

    /// Reads the file to the `target` `Vec` if it exists.
    ///
    /// Returns `Ok(Some(file_content_len))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_to_vec_if_exists(&self, target: &mut Vec<u8>) -> Result<Option<usize>, Error> {
        if let Some(local) = LocalPath::parse(self.path()) {
            return local.read_to_vec_if_exists(target);
        }

        Err(Error::new(self.clone(), Read, UnknownFileSystem))
    }
}
