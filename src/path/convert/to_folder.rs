use crate::{Error, FolderPath, Operation, Reason, StoragePath};

impl StoragePath {
    //! To Folder

    /// Converts the path to a folder path.
    ///
    /// # Errors
    /// - Fails with `InvalidPath` if the path is a file.
    pub fn to_folder(self) -> Result<FolderPath, Error> {
        if self.is_folder() {
            Ok(unsafe { FolderPath::new(self) })
        } else {
            Err(Error::new(
                self,
                Operation::ConvertPath,
                Reason::InvalidPath,
            ))
        }
    }

    /// Clones the path and converts it to a folder path.
    ///
    /// # Errors
    /// - Fails with `InvalidPath` if the path is a file.
    pub fn clone_to_folder(&self) -> Result<FolderPath, Error> {
        self.clone().to_folder()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FolderPath, Operation, Reason, StoragePath};

    #[test]
    fn to_folder() {
        for path in ["/folder/", "/", "C:\\folder\\", "C:\\"] {
            let folder: FolderPath = StoragePath::parse(path).unwrap().to_folder().unwrap();
            assert_eq!(folder.as_str(), path);
        }
    }

    #[test]
    fn to_folder_file_is_err() {
        for path in ["/file.txt", "C:\\folder\\file.txt"] {
            let path: StoragePath = StoragePath::parse(path).unwrap();
            let error: Error = path.clone().to_folder().unwrap_err();
            assert_eq!(error.path(), &path);
            assert_eq!(error.operation(), Operation::ConvertPath);
            assert_eq!(error.reason(), Reason::InvalidPath);
        }
    }

    #[test]
    fn clone_to_folder() {
        let path: StoragePath = StoragePath::parse("/folder/").unwrap();
        let folder: FolderPath = path.clone_to_folder().unwrap();
        assert_eq!(folder.as_str(), "/folder/");
        assert_eq!(path.as_str(), "/folder/");
    }

    #[test]
    fn clone_to_folder_file_is_err() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        assert!(path.clone_to_folder().is_err());
    }
}
