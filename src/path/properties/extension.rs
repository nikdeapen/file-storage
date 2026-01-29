use crate::StoragePath;

impl StoragePath {
    //! Extension

    /// Gets the path extension.
    pub fn extension(&self) -> &str {
        &self.path()[self.base_len()..]
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn base_and_extension() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.extension(), "the/path");
    }
}
