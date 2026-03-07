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

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn to_folder() {
        let path: StoragePath = StoragePath::parse("/folder/").unwrap();
        assert!(path.to_folder().is_ok());
    }

    #[test]
    fn to_folder_root() {
        let path: StoragePath = StoragePath::parse("/").unwrap();
        assert!(path.to_folder().is_ok());
    }

    #[test]
    fn to_folder_file_is_err() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        assert!(path.to_folder().is_err());
    }

    #[test]
    fn clone_to_folder() {
        let path: StoragePath = StoragePath::parse("/folder/").unwrap();
        let folder = path.clone_to_folder().unwrap();
        assert_eq!(folder.as_str(), "/folder/");
        assert_eq!(path.as_str(), "/folder/");
    }
}
