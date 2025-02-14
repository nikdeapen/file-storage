use std::io;
use std::io::ErrorKind::Other;

use crate::Operation::Read;
use crate::Reason::FileNotFound;
use crate::{Error, FilePath};

impl FilePath {
    //! Read as String

    /// Reads the file as a `String`.
    ///
    /// Returns `Ok(file_content)`.
    /// Returns `Err(FileNotFound)` if the file did not exist.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(file_content) = self.read_as_string_if_exists()? {
            Ok(file_content)
        } else {
            Err(Error::new(self.clone(), Read, FileNotFound))
        }
    }

    /// Reads the file as a `String` if it exists.
    ///
    /// Returns `Ok(Some(file_content))`.
    /// Returns `Ok(None)` if the file did not exist.
    pub fn read_as_string_if_exists(&self) -> Result<Option<String>, Error> {
        if let Some(file_content) = self.read_as_vec_if_exists()? {
            match String::from_utf8(file_content) {
                Ok(file_content) => Ok(Some(file_content)),
                Err(error) => Err(Error::from_cause(
                    self.clone(),
                    Read,
                    io::Error::new(Other, error),
                )),
            }
        } else {
            Ok(None)
        }
    }
}
