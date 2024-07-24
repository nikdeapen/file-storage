use crate::{FilePath, FolderPath};

impl FilePath {
    //! Length

    /// Gets the path length.
    pub fn len(&self) -> usize {
        self.path().len()
    }
}

impl FolderPath {
    //! Length

    /// Gets the path length.
    pub fn len(&self) -> usize {
        self.path().len()
    }
}
