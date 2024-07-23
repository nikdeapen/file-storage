use crate::Path;

impl Path {
    //! Clone

    /// Clones the path with extra capacity.
    pub fn clone_with_extra_capacity(&self, extra_capacity: usize) -> Self {
        let mut path: String = String::with_capacity(self.path().len() + extra_capacity);
        path.push_str(self.path());
        unsafe { Self::new_unchecked(path, self.base_len(), self.file_separator()) }
    }
}
