use std::io::ErrorKind::AlreadyExists;
use std::io::Write as IOWrite;

use crate::Operation::Write;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Write Data If Not Exists

    /// Writes the `data` to the file if the file does not already exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already existed.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if self.is_local_path() {
            return self.write_data_if_not_exists_local(data);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}

impl FilePath {
    //! Local

    /// See `FilePath::write_data_if_not_exists`.
    fn write_data_if_not_exists_local<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        debug_assert!(self.is_local_path());

        if let Some(parent) = std::path::Path::new(self).parent() {
            match std::fs::create_dir_all(parent) {
                Ok(()) => {}
                Err(error) => return Err(Error::from_cause(self.clone(), Write, error)),
            }
        }
        match std::fs::File::create_new(self) {
            Ok(mut file) => {
                file.write_all(data.as_ref())
                    .map_err(|e| Error::from_cause(self.clone(), Write, e))?;
                file.sync_all()
                    .map_err(|e| Error::from_cause(self.clone(), Write, e))?;
                Ok(true)
            }
            Err(error) => {
                if error.kind() == AlreadyExists {
                    Ok(false)
                } else {
                    Err(Error::from_cause(self.clone(), Write, error))
                }
            }
        }
    }
}
