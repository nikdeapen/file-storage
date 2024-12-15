use crate::path::StoragePath;

/// A file path.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FilePath {
    path: StoragePath,
}

impl FilePath {
    //! Construction

    /// Creates a file path from the `path`.
    ///
    /// # Unsafe
    /// The `path` must be a file path.
    pub unsafe fn new_unchecked(path: StoragePath) -> Self {
        debug_assert!(path.is_file());

        Self { path }
    }
}

impl FilePath {
    //! Path

    /// Gets the path.
    pub fn path(&self) -> &StoragePath {
        &self.path
    }

    /// Converts the file path to a path.
    pub fn to_path(self) -> StoragePath {
        self.path
    }
}
