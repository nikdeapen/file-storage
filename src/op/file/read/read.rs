use crate::system::LocalPath;
use crate::{Error, FilePath, FileRead, Operation, Reason};

impl FilePath {
    //! Read

    /// Reads the file if it exists.
    pub fn read_if_exists(&self) -> Result<Option<FileRead>, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.read_if_exists();
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.read_if_exists();
        }

        Err(Error::new(
            self.clone(),
            Operation::Read,
            Reason::UnknownFileSystem,
        ))
    }

    /// Reads the file.
    pub fn read(&self) -> Result<FileRead, Error> {
        if let Some(read) = self.read_if_exists()? {
            Ok(read)
        } else {
            Err(Error::new(
                self.clone(),
                Operation::Read,
                Reason::FileNotFound,
            ))
        }
    }
}
