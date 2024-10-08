use crate::StoragePath;

impl StoragePath {
    //! File or Folder

    /// Checks if the path is a file. (if not, the path is a folder)
    pub fn is_file(&self) -> bool {
        !self.is_folder()
    }

    /// Checks if the path is a folder. (if not, the path is a file)
    pub fn is_folder(&self) -> bool {
        self.extension().len() == 0 || self.path().ends_with(self.file_separator())
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_file() {
        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/file", 1, '/') };
        assert!(path.is_file());

        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/folder/", 1, '/') };
        assert!(!path.is_file());
    }

    #[test]
    fn is_folder() {
        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/file", 1, '/') };
        assert!(!path.is_folder());

        let path: StoragePath = unsafe { StoragePath::new_unchecked("/the/folder/", 1, '/') };
        assert!(path.is_folder());
    }
}
