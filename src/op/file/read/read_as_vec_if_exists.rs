use crate::{Error, FilePath};

impl FilePath {
    //! Read To Vec If Exists

    /// Reads the file as a `Vec<u8>`.
    ///
    /// Returns `Ok(Some(vec))` with the number of bytes read.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_as_vec_if_exists(&self) -> Result<Option<Vec<u8>>, Error> {
        let mut target: Vec<u8> = Vec::default();
        if let Some(read) = self.read_to_vec_if_exists(&mut target)? {
            debug_assert_eq!(read, target.len());
            Ok(Some(target))
        } else {
            Ok(None)
        }
    }
}
