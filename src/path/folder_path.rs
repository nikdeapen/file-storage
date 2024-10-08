use crate::path::StoragePath;

/// A folder path.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FolderPath {
    path: StoragePath,
}

impl FolderPath {
    //! Construction

    /// Creates a folder path from the `path`.
    ///
    /// # Unsafe
    /// The `path` must be a folder path.
    pub unsafe fn new_unchecked(path: StoragePath) -> Self {
        debug_assert!(path.is_folder());

        Self { path }
    }
}

impl FolderPath {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> &StoragePath {
        &self.path
    }

    /// Converts the folder path to a path.
    pub fn to_path(self) -> StoragePath {
        self.path
    }
}
