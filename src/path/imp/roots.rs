use crate::Path;

impl Path {
    //! Roots

    /// Gets the unix root path.
    pub fn unix_root() -> Self {
        unsafe { Self::new_unchecked("/", 1, '/') }
    }
}
