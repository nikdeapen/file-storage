use crate::{FolderPath, Path};

impl Path {
    //! Roots

    /// Gets the unix root path.
    #[cfg(feature = "local")]
    pub fn unix_root() -> Self {
        unsafe { Self::new_unchecked("/", 1, '/') }
    }
}

impl FolderPath {
    //! Roots

    /// Gets the unix root path.
    #[cfg(feature = "local")]
    pub fn unix_root() -> Self {
        Path::unix_root().to_folder().unwrap()
    }
}

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FolderPath, Path};

    #[test]
    fn unix_root() {
        assert_eq!(Path::unix_root().path(), "/");
        assert_eq!(FolderPath::unix_root().path().path(), "/");
    }
}
