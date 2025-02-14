use crate::system::LocalPath;
use crate::Operation::Write;
use crate::Reason::{FileAlreadyExists, UnknownFileSystem};
use crate::{Error, FilePath};

impl FilePath {
    //! Write Data

    /// Writes the `slice` to the file.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_slice<D>(&self, slice: D) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if self.write_slice_if_not_exists(slice)? {
            Ok(())
        } else {
            Err(Error::new(self.clone(), Write, FileAlreadyExists))
        }
    }

    /// Writes the `slice` to the file if the file does not already exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already exists.
    pub fn write_slice_if_not_exists<D>(&self, slice: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.write_slice_if_not_exists(slice);
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.write_slice_if_not_exists(slice);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}
