use crate::Operation::Write;
#[cfg(feature = "r2")]
use crate::R2Path;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath, LocalPath};

impl FilePath {
    //! Write Data

    /// Writes the `data` to a new file.
    ///
    /// # Note
    /// - It fails with `FileAlreadyExists` if the file already exists.
    /// - The file is created atomically. See `write_data_if_not_exists`.
    pub fn write_data<D>(&self, data: D) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if self.write_data_if_not_exists(data)? {
            Ok(())
        } else {
            Err(Error::new(self.clone(), Write, FileAlreadyExists))
        }
    }

    /// Writes the `data` to a new file if the file does not exist.
    ///
    /// # Atomic
    /// - Only one writer can create the file. The others get `false`.
    ///
    /// # Note
    /// - It is `true` if the file was written. It is `false` if the file already exists.
    /// - A local folder at the path is an error.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(path) = LocalPath::new(self.path()) {
            return path.write_data_if_not_exists(data.as_ref());
        }

        #[cfg(feature = "r2")]
        if let Some(path) = R2Path::new(self.path()) {
            return path.write_data_if_not_exists(data);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}
