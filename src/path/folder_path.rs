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
    /// # Safety
    /// The `path` must be a valid folder path.
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
