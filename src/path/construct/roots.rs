use crate::{FolderPath, StoragePath};

impl StoragePath {
    //! Roots

    /// Creates the unix root path.
    pub fn unix_root() -> Self {
        unsafe { Self::new("/", 1, '/') }
    }
}

impl FolderPath {
    //! Roots

    /// Creates the unix root folder.
    pub fn unix_root() -> Self {
        StoragePath::unix_root().make_folder()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FolderPath, StoragePath};

    #[test]
    fn unix_root() {
        let path: StoragePath = StoragePath::unix_root();
        assert_eq!(path.as_str(), "/");
        assert_eq!(path.base_path(), "/");
        assert_eq!(path.relative_path(), "");
        assert_eq!(path.file_separator(), '/');
        assert!(path.is_folder());
    }

    #[test]
    fn unix_root_folder() {
        let folder: FolderPath = FolderPath::unix_root();
        assert_eq!(folder.path(), &StoragePath::unix_root());
    }
}
