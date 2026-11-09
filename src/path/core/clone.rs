use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Clone

    /// Clones the path with room for `extra_capacity` more bytes.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let mut path: String = String::with_capacity(self.len() + extra_capacity);
        path.push_str(self.path());
        unsafe { Self::new(path, self.base_len(), self.file_separator()) }
    }
}

impl FilePath {
    //! Clone

    /// Clones the path with room for `extra_capacity` more bytes.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let path: StoragePath = self.path().clone_with_extra_capacity(extra_capacity);
        unsafe { Self::new(path) }
    }
}

impl FolderPath {
    //! Clone

    /// Clones the path with room for `extra_capacity` more bytes.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let path: StoragePath = self.path().clone_with_extra_capacity(extra_capacity);
        unsafe { Self::new(path) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath, StoragePath};

    #[test]
    fn clone_with_extra_capacity() {
        let path: StoragePath = StoragePath::parse("C:\\folder\\file.txt").unwrap();
        let mut clone: StoragePath = path.clone_with_extra_capacity(10);
        assert_eq!(clone, path);
        assert_eq!(clone.base_len(), path.base_len());
        assert_eq!(clone.file_separator(), path.file_separator());
        assert!(unsafe { clone.path_mut() }.capacity() >= path.len() + 10);
    }

    #[test]
    fn clone_file_and_folder_with_extra_capacity() {
        let file: FilePath = StoragePath::parse("/folder/file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.clone_with_extra_capacity(4), file);

        let folder: FolderPath = StoragePath::parse("/folder/").unwrap().to_folder().unwrap();
        assert_eq!(folder.clone_with_extra_capacity(4), folder);
    }
}
