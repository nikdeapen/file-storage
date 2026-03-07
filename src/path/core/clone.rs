use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Clone

    /// Clones the path with the extra capacity.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let mut path: String = String::with_capacity(self.len() + extra_capacity);
        path.push_str(self.path());
        unsafe { Self::new(path, self.base_len(), self.file_separator()) }
    }
}

impl FilePath {
    //! Clone

    /// Clones the path with the extra capacity.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let path: StoragePath = self.path().clone_with_extra_capacity(extra_capacity);
        // Safety: the original path is already a file path.
        unsafe { Self::new(path) }
    }
}

impl FolderPath {
    //! Clone

    /// Clones the path with the extra capacity.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let path: StoragePath = self.path().clone_with_extra_capacity(extra_capacity);
        // Safety: the original path is already a folder path.
        unsafe { Self::new(path) }
    }
}
