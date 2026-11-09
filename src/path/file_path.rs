use crate::StoragePath;

/// A file path.
#[must_use]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FilePath {
    path: StoragePath,
}

impl FilePath {
    //! Construction

    /// Creates the [FilePath].
    ///
    /// # Safety
    /// - The `path` must be a file path. See `StoragePath::is_file`.
    pub unsafe fn new(path: StoragePath) -> Self {
        Self { path }
    }
}

impl FilePath {
    //! Storage Path

    /// Gets the storage path.
    pub fn path(&self) -> &StoragePath {
        &self.path
    }

    /// Converts the file path to a storage path.
    pub fn to_path(self) -> StoragePath {
        self.path
    }
}
