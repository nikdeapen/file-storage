use crate::StoragePath;

impl StoragePath {
    //! Base

    /// Gets the path base.
    pub fn base(&self) -> &str {
        &self.path()[..self.base_len()]
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn base_and_extension() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.base(), "/");
    }
}
