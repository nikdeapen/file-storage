use crate::{Error, FilePath};

impl FilePath {
    //! Read as Vec If Exists

    /// Reads the file as a `Vec<u8>` if it exists.
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
