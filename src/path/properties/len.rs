use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }
}

impl FilePath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }
}

impl FolderPath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }
}
