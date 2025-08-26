use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }

    /// Checks if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl FilePath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }

    /// Checks if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl FolderPath {
    //! Length

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path().len()
    }

    /// Checks if the path is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn len() {
        let path: StoragePath = unsafe { StoragePath::new("/你/好", 1, '/') };
        assert_eq!(path.len(), 8);
    }
}
