use crate::{Error, FilePath, Operation, Reason, StoragePath};

impl StoragePath {
    //! To File

    /// Converts the path to a file path.
    ///
    /// # Errors
    /// - Fails with `InvalidPath` if the path is a folder.
    pub fn to_file(self) -> Result<FilePath, Error> {
        if self.is_file() {
            Ok(unsafe { FilePath::new(self) })
        } else {
            Err(Error::new(
                self,
                Operation::ConvertPath,
                Reason::InvalidPath,
            ))
        }
    }

    /// Clones the path and converts it to a file path.
    ///
    /// # Errors
    /// - Fails with `InvalidPath` if the path is a folder.
    pub fn clone_to_file(&self) -> Result<FilePath, Error> {
        self.clone().to_file()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FilePath, Operation, Reason, StoragePath};

    #[test]
    fn to_file() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        let file: FilePath = path.to_file().unwrap();
        assert_eq!(file.as_str(), "/file.txt");
    }

    #[test]
    fn to_file_windows() {
        let path: StoragePath = StoragePath::parse("C:\\folder\\file.txt").unwrap();
        let file: FilePath = path.to_file().unwrap();
        assert_eq!(file.as_str(), "C:\\folder\\file.txt");
    }

    #[test]
    fn to_file_folder_is_err() {
        for path in ["/folder/", "/", "C:\\folder\\", "C:\\"] {
            let path: StoragePath = StoragePath::parse(path).unwrap();
            let error: Error = path.clone().to_file().unwrap_err();
            assert_eq!(error.path(), &path);
            assert_eq!(error.operation(), Operation::ConvertPath);
            assert_eq!(error.reason(), Reason::InvalidPath);
        }
    }

    #[test]
    fn clone_to_file() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        let file: FilePath = path.clone_to_file().unwrap();
        assert_eq!(file.as_str(), "/file.txt");
        assert_eq!(path.as_str(), "/file.txt");
    }

    #[test]
    fn clone_to_file_folder_is_err() {
        let path: StoragePath = StoragePath::parse("/folder/").unwrap();
        assert!(path.clone_to_file().is_err());
    }
}
