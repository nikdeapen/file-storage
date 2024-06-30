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
#[cfg(feature = "local")]
mod tests {
    use crate::Path;

    #[test]
    fn base_and_extension() {
        let path: Path = Path::unix_root().with_appended("the/path");
        assert_eq!(path.base(), "/");
        assert_eq!(path.extension(), "the/path");
    }
}
