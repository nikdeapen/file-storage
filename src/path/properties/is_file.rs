use crate::StoragePath;

impl StoragePath {
    //! Is-File

    /// Checks if the path is a file. (if not, the path is a folder)
    pub fn is_file(&self) -> bool {
        !self.is_folder()
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_file_or_folder() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert!(path.is_file());

        let path: StoragePath = unsafe { StoragePath::new("/the/path/", 1, '/') };
        assert!(!path.is_file());

        let path: StoragePath = unsafe { StoragePath::new("!", 1, '/') };
        assert!(!path.is_file());
    }
}
