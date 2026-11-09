use crate::StoragePath;

/// A folder path.
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FolderPath {
    path: StoragePath,
}

impl FolderPath {
    //! Construction

    /// Creates the [FolderPath].
    ///
    /// # Safety
    /// - The `path` must be a folder path. See `StoragePath::is_folder`.
    pub unsafe fn new(path: StoragePath) -> Self {
        Self { path }
    }
}

impl FolderPath {
    //! Storage Path

    /// Gets the storage path.
    pub fn path(&self) -> &StoragePath {
        &self.path
    }

    /// Converts the folder path to a storage path.
    pub fn to_path(self) -> StoragePath {
        self.path
    }
}
