use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Truncate

    /// Truncates the path to the new length.
    ///
    /// # Unsafe
    /// The `new_len` must be >= to the `base_len` and must be a valid char boundary.
    pub unsafe fn truncate(&mut self, new_len: usize) {
        debug_assert!(new_len > self.base_len());
        debug_assert!(self.path().is_char_boundary(new_len));

        self.path_mut().truncate(new_len)
    }

    /// Truncates the path to the new length.
    ///
    /// # Unsafe
    /// The `new_len` must be >= to the `base_len` and must be a valid char boundary.
    pub unsafe fn truncated(mut self, new_len: usize) -> Self {
        debug_assert!(new_len > self.base_len());
        debug_assert!(self.path().is_char_boundary(new_len));

        self.truncate(new_len);
        self
    }
}

impl FilePath {
    /// Parent

    /// Converts the file path to the parent folder.
    pub fn to_parent(self) -> FolderPath {
        let file_name: &str = self.file_name();
        let new_len: usize = self.path().len() - file_name.len();
        let path: Path = self.to_path();
        let path: Path = unsafe { path.truncated(new_len) };
        path.to_folder().unwrap()
    }

    /// Gets the parent folder.
    pub fn parent(&self) -> FolderPath {
        self.clone().to_parent()
    }
}
