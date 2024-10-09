use std::io::ErrorKind::AlreadyExists;
use std::io::Write as IOWrite;

use crate::Operation::Write;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Write Data If Not Exists

    /// Writes the `data` to the file if the file does not exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already existed.
    pub fn write_data_if_not_exists(&self, data: &[u8]) -> Result<bool, Error> {
        if self.is_local_path() {
            self.write_data_if_not_exists_local(data)
        } else {
            Err(Error::new(self, Write, UnknownFileSystem))
        }
    }
}

impl FilePath {
    //! Write Slice If Not Exists - Local

    pub fn write_data_if_not_exists_local(&self, data: &[u8]) -> Result<bool, Error> {
        debug_assert!(self.is_local_path());

        if let Some(parent) = std::path::Path::new(self).parent() {
            match std::fs::create_dir_all(parent) {
                Ok(()) => {}
                Err(error) => return Err(Error::from_error(self, Write, error)),
            }
        }
        match std::fs::File::create_new(self) {
            Ok(mut file) => {
                file.write_all(data)
                    .map_err(|e| Error::from_error(self, Write, e))?;
                file.sync_all()
                    .map_err(|e| Error::from_error(self, Write, e))?;
                Ok(true)
            }
            Err(error) => {
                if error.kind() == AlreadyExists {
                    Ok(false)
                } else {
                    Err(Error::from_error(self, Write, error))
                }
            }
        }
    }
}
