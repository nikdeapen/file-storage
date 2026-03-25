use crate::system::LocalPath;
use crate::Operation;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath};
use std::io::Write;

impl FilePath {
    //! Write Data

    /// Writes the `data` to the file.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_data<D>(&self, data: D) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if self.write_data_if_not_exists(data)? {
            Ok(())
        } else {
            Err(Error::new(
                self.clone(),
                Operation::Write,
                FileAlreadyExists,
            ))
        }
    }

    /// Writes the `data` to the file if the file does not already exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(path) = LocalPath::new(self.path()) {
            return if let Some(mut write) = path.write_if_not_exists()? {
                write
                    .write_all(data.as_ref())
                    .map_err(|e| Error::from_source(self.clone(), Operation::Write, e))?;
                write
                    .close()
                    .map_err(|e| Error::from_source(self.clone(), Operation::Write, e))?;
                Ok(true)
            } else {
                Ok(false)
            };
        }

        #[cfg(feature = "r2")]
        if let Some(path) = crate::R2Path::new(self.path()) {
            return path.write_data_if_not_exists(data);
        }

        Err(Error::new(
            self.clone(),
            Operation::Write,
            UnknownFileSystem,
        ))
    }
}
