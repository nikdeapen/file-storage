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
