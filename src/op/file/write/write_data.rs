use crate::Operation::Write;
use crate::Reason::FileAlreadyExists;
use crate::{Error, FilePath};

impl FilePath {
    //! Write Data

    /// Writes the `data`.
    ///
    /// Returns `Ok(())`.
    /// Returns `Err(FileAlreadyExists)` if the file already exists.
    pub fn write_data<D>(&self, data: D) -> Result<(), Error>
    where
        D: AsRef<[u8]>,
    {
        if self.write_data_if_not_exists(data.as_ref())? {
            Ok(())
        } else {
            Err(Error::new(self, Write, FileAlreadyExists))
        }
    }
}
