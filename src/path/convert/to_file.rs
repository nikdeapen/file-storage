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

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn to_file() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        assert!(path.to_file().is_ok());
    }

    #[test]
    fn to_file_folder_is_err() {
        let path: StoragePath = StoragePath::parse("/folder/").unwrap();
        assert!(path.to_file().is_err());
    }

    #[test]
    fn to_file_root_is_err() {
        let path: StoragePath = StoragePath::parse("/").unwrap();
        assert!(path.to_file().is_err());
    }

    #[test]
    fn clone_to_file() {
        let path: StoragePath = StoragePath::parse("/file.txt").unwrap();
        let file = path.clone_to_file().unwrap();
        assert_eq!(file.as_str(), "/file.txt");
        assert_eq!(path.as_str(), "/file.txt");
    }
}
