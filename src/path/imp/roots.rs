use crate::{FolderPath, Path};

impl Path {
    //! Roots

    /// Gets the unix root path.
    pub fn unix_root() -> Self {
        unsafe { Self::new_unchecked("/", 1, '/') }
    }
}

impl FolderPath {
    //! Roots

    /// Gets the unix root path.
    pub fn unix_root() -> Self {
        Path::unix_root()
            .to_folder()
            .expect("this must be a folder")
    }
}
