use crate::StoragePath;

impl StoragePath {
    //! Is-Folder

    /// Checks if the path is a folder. (if not, the path is a file)
    pub fn is_folder(&self) -> bool {
        self.extension().is_empty() || self.path().ends_with(self.file_separator())
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_file_or_folder() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert!(!path.is_folder());

        let path: StoragePath = unsafe { StoragePath::new("/the/path/", 1, '/') };
        assert!(path.is_folder());

        let path: StoragePath = unsafe { StoragePath::new("!", 1, '/') };
        assert!(path.is_folder());
    }
}
