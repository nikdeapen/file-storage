use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! To File or Folder

    /// Converts the path to a file path.
    ///
    /// Returns `Err(self)` if the path is not a file path.
    pub fn to_file(self) -> Result<FilePath, StoragePath> {
        if self.is_file() {
            Ok(unsafe { FilePath::new(self) })
        } else {
            Err(self)
        }
    }

    /// Converts the path to a folder path.
    ///
    /// Returns `Err(self)` if the path is not a folder path.
    pub fn to_folder(self) -> Result<FolderPath, StoragePath> {
        if self.is_folder() {
            Ok(unsafe { FolderPath::new(self) })
        } else {
            Err(self)
        }
    }
}
