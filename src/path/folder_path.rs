use crate::path::Path;

/// A folder path.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FolderPath {
    path: Path,
}

impl FolderPath {
    //! Construction

    /// Creates a folder path from the path.
    ///
    /// # Unsafe
    /// The path must be a folder path.
    pub unsafe fn new_unchecked(path: Path) -> Self {
        debug_assert!(path.is_folder());

        Self { path }
    }
}

impl FolderPath {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Converts the folder path to a path.
    pub fn to_path(self) -> Path {
        self.path
    }
}
