use crate::StoragePath;

impl StoragePath {
    //! Base & Extension

    /// Gets the path base.
    pub fn base(&self) -> &str {
        &self.path()[..self.base_len()]
    }

    /// Gets the path extension.
    pub fn extension(&self) -> &str {
        &self.path()[self.base_len()..]
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn base_extension() {
        let s: &str = "/the/path";
        let path: StoragePath = unsafe { StoragePath::new_unchecked(s, 1, '/') };

        assert_eq!(path.base(), "/");
        assert_eq!(path.extension(), "the/path");
    }
}
