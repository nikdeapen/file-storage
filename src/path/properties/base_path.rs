use crate::StoragePath;

impl StoragePath {
    //! Base Path

    /// Gets the base path.
    pub fn base_path(&self) -> &str {
        &self.path()[..self.base_len()]
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn base() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.base_path(), "/");
    }
}
