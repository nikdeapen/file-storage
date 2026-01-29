use crate::{Error, FilePath, Operation, Reason, StoragePath};

impl StoragePath {
    //! To File

    /// Converts the path to a file path.
    ///
    /// Returns `Err(self)` if the path is not a file path.
    pub fn to_file(self) -> Result<FilePath, Error> {
        if self.is_file() {
            Ok(unsafe { FilePath::new(self) })
        } else {
            Err(Error::new(self, Operation::ModifyPath, Reason::Other))
        }
    }
}
