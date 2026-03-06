use crate::{Error, FolderPath, Operation, Reason, StoragePath};

impl StoragePath {
    //! To Folder

    /// Converts the path to a folder path.
    ///
    /// Returns `Err` if the path is not a folder path.
    pub fn to_folder(self) -> Result<FolderPath, Error> {
        if self.is_folder() {
            Ok(unsafe { FolderPath::new(self) })
        } else {
            Err(Error::new(self, Operation::ModifyPath, Reason::InvalidPath))
        }
    }

    /// Clones the path and converts it to a folder path.
    ///
    /// Returns `Err` if the path is not a folder path.
    pub fn clone_to_folder(&self) -> Result<FolderPath, Error> {
        self.clone().to_folder()
    }
}
