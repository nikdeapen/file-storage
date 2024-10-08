use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! To File or Folder

    /// Converts the path to a file path.
    ///
    /// Returns `None` if the path is not a file path.
    pub fn to_file(self) -> Option<FilePath> {
        if self.is_file() {
            Some(unsafe { FilePath::new_unchecked(self) })
        } else {
            None
        }
    }

    /// Converts the path to a folder path.
    ///
    /// Returns `None` if the path is not a folder path.
    pub fn to_folder(self) -> Option<FolderPath> {
        if self.is_folder() {
            Some(unsafe { FolderPath::new_unchecked(self) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn to_file() {
        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/file", 1, '/') };
        assert!(path.to_file().is_some());

        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/folder/", 1, '/') };
        assert!(path.to_file().is_none());
    }

    #[test]
    fn to_folder() {
        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/file", 1, '/') };
        assert!(path.to_folder().is_none());

        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/folder/", 1, '/') };
        assert!(path.to_folder().is_some());
    }
}
