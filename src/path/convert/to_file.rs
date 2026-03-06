use crate::{Error, FilePath, Operation, Reason, StoragePath};

impl StoragePath {
    //! To File

    /// Converts the path to a file path.
    ///
    /// Returns `Err` if the path is not a file path.
    pub fn to_file(self) -> Result<FilePath, Error> {
        if self.is_file() {
            Ok(unsafe { FilePath::new(self) })
        } else {
            Err(Error::new(self, Operation::ModifyPath, Reason::InvalidPath))
        }
    }

    /// Clones the path and converts it to a file path.
    ///
    /// Returns `Err` if the path is not a file path.
    pub fn clone_to_file(&self) -> Result<FilePath, Error> {
        self.clone().to_file()
    }
}
