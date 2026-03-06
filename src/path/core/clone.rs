use crate::StoragePath;

impl StoragePath {
    //! Clone

    /// Clones the path with the extra capacity.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let mut path: String = String::with_capacity(self.len() + extra_capacity);
        path.push_str(self.path());
        unsafe { Self::new(path, self.base_len(), self.file_separator()) }
    }
}
