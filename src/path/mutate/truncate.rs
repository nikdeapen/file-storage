use crate::StoragePath;

impl StoragePath {
    //! Truncate

    /// Truncates the path to the `new_len`.
    ///
    /// # Safety
    /// The `new_len` must be >= to the `base_len` and must be a valid char boundary.
    pub unsafe fn truncate(&mut self, new_len: usize) {
        debug_assert!(new_len >= self.base_len());
        debug_assert!(self.path().is_char_boundary(new_len));

        unsafe { self.path_mut() }.truncate(new_len)
    }

    /// Truncates the path to the `new_len`.
    ///
    /// # Safety
    /// The `new_len` must be >= to the `base_len` and must be a valid char boundary.
    pub unsafe fn truncated(mut self, new_len: usize) -> Self {
        debug_assert!(new_len >= self.base_len());
        debug_assert!(self.path().is_char_boundary(new_len));

        self.truncate(new_len);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn truncate() {
        let path: StoragePath = StoragePath::unix_root().with_appended("folder.txt");

        let path: StoragePath = unsafe { path.truncated(7) };
        assert_eq!(path.as_str(), "/folder");

        let path: StoragePath = unsafe { path.truncated(1) };
        assert_eq!(path.as_str(), "/");
    }
}
