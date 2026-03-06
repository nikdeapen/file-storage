use crate::StoragePath;
use std::ops::Deref;

/// A file path.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FilePath {
    path: StoragePath,
}

impl Deref for FilePath {
    type Target = StoragePath;

    fn deref(&self) -> &StoragePath {
        &self.path
    }
}

impl FilePath {
    //! Construction

    /// Creates a file path from the `path`.
    ///
    /// # Safety
    /// The `path` must be a valid file path.
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
