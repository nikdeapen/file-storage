use crate::system::LocalPath;
use crate::Operation::Write;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath};

impl FilePath {
    //! Write Data If-Not-Exists

    /// Writes the `data` to the file if the file does not already exist.
    ///
    /// Returns `Ok(true)` if the file was written.
    /// Returns `Ok(false)` if the file already existed.
    pub fn write_data_if_not_exists<D>(&self, data: D) -> Result<bool, Error>
    where
        D: AsRef<[u8]>,
    {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.write_data_if_not_exists(data);
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.write_data_if_not_exists(data);
        }

        Err(Error::new(self.clone(), Write, UnknownFileSystem))
    }
}
