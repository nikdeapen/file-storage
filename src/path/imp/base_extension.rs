use crate::path::Path;

impl Path {
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
    use crate::Path;

    #[test]
    fn base_and_extension() {
        let path: Path = unsafe { Path::new_unchecked("/the/path", 1, '/') };
        assert_eq!(path.base(), "/");
        assert_eq!(path.extension(), "the/path");
    }
}
