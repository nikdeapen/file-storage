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

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn len() {
        let path: StoragePath = unsafe { StoragePath::new_unchecked("/你/好", 1, '/') };
        assert_eq!(path.len(), 8);
    }
}
