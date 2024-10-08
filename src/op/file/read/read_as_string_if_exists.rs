use std::io;
use std::io::ErrorKind::Other;

use crate::Operation::Read;
use crate::{Error, FilePath};

impl FilePath {
    //! Read to String If Exists

    /// Reads the file as a `String` if it exists.
    ///
    /// Returns `Ok(Some(string))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_as_string_if_exists(&self) -> Result<Option<String>, Error> {
        if let Some(data) = self.read_as_vec_if_exists()? {
            match String::from_utf8(data) {
                Ok(s) => Ok(Some(s)),
                Err(e) => Err(Error::from_error(self, Read, io::Error::new(Other, e))),
            }
        } else {
            Ok(None)
        }
    }
}
