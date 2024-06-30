use std::io::ErrorKind::Other;
use std::io::Read;

use crate::op::FileRead;
use crate::ErrorInfo::{FileNotFound, UnknownFileSystem};
use crate::{Error, FilePath, Operation};

impl FilePath {
    //! Read

    /// Reads the file.
    ///
    /// # Errors
    /// `FileNotFound` - if the file does not exist.
    pub fn read(&self) -> Result<FileRead, Error> {
        if let Some(read) = self.read_if_exists()? {
            Ok(read)
        } else {
            Err(Error::new(self, Operation::Read, FileNotFound))
        }
    }

    /// Reads the file if it exists.
    pub fn read_if_exists(&self) -> Result<Option<FileRead>, Error> {
        #[cfg(feature = "local")]
        if crate::local::is_local_path(self) {
            return crate::local::read_if_exists(self);
        }

        Err(Error::new(self, Operation::Read, UnknownFileSystem))
    }

    /// Reads the file as a `Vec<u8>`.
    ///
    /// # Errors
    /// `FileNotFound` - if the file does not exist.
    pub fn read_as_vec(&self) -> Result<Vec<u8>, Error> {
        if let Some(read) = self.read_as_vec_if_exists()? {
            Ok(read)
        } else {
            Err(Error::new(self, Operation::Read, FileNotFound))
        }
    }

    /// Reads the file as a `Vec<u8>` if it exists.
    pub fn read_as_vec_if_exists(&self) -> Result<Option<Vec<u8>>, Error> {
        if let Some(mut read) = self.read_if_exists()? {
            let mut vec: Vec<u8> = Vec::default();
            read.read_to_end(&mut vec)
                .map_err(|error| Error::from_error(self, Operation::Read, error))?;
            Ok(Some(vec))
        } else {
            Ok(None)
        }
    }

    /// Reads the file as a `String`.
    ///
    /// # Errors
    /// `FileNotFound` - if the file does not exist.
    pub fn read_as_string(&self) -> Result<String, Error> {
        if let Some(content) = self.read_as_string_if_exists()? {
            Ok(content)
        } else {
            Err(Error::new(self, Operation::Read, FileNotFound))
        }
    }

    /// Reads the file as a `String` if it exists.
    pub fn read_as_string_if_exists(&self) -> Result<Option<String>, Error> {
        if let Some(vec) = self.read_as_vec_if_exists()? {
            Ok(Some(String::from_utf8(vec).map_err(|error| {
                Error::from_error(self, Operation::Read, std::io::Error::new(Other, error))
            })?))
        } else {
            Ok(None)
        }
    }
}
