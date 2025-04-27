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
    fn is_file_or_folder() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert!(path.is_file());
        assert!(!path.is_folder());

        let path: StoragePath = unsafe { StoragePath::new("/the/path/", 1, '/') };
        assert!(!path.is_file());
        assert!(path.is_folder());

        let path: StoragePath = unsafe { StoragePath::new("!", 1, '/') };
        assert!(!path.is_file());
        assert!(path.is_folder());
    }
}
