use crate::StoragePath;

impl StoragePath {
    //! Relative Path

    /// Gets the relative path (the portion after the base).
    pub fn relative_path(&self) -> &str {
        &self.path()[self.base_len()..]
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn relative_path() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.relative_path(), "the/path");
    }
}
