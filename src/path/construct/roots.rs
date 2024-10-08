use crate::{FolderPath, StoragePath};

impl StoragePath {
    //! Roots

    /// Creates the unix root path.
    pub fn unix_root() -> Self {
        unsafe { Self::new_unchecked("/", 1, '/') }
    }
}

impl FolderPath {
    //! Roots

    /// Creates the unix root path.
    pub fn unix_root() -> Self {
        StoragePath::unix_root().make_folder()
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn unix_root() {
        let path: StoragePath = StoragePath::unix_root();
        debug_assert_eq!(path.path(), "/");
        debug_assert_eq!(path.base_len(), 1);
        debug_assert_eq!(path.file_separator(), '/');
    }
}
