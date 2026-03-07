use crate::StoragePath;

impl StoragePath {
    //! Is System

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        Self::is_unix_path_str(self.path())
    }

    /// Checks if the `path` is a Unix path.
    pub fn is_unix_path_str<S>(path: S) -> bool
    where
        S: AsRef<str>,
    {
        path.as_ref().starts_with('/')
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_unix_path_str() {
        assert!(StoragePath::is_unix_path_str("/"));
        assert!(StoragePath::is_unix_path_str("/usr/bin"));
        assert!(StoragePath::is_unix_path_str("/file.txt"));

        assert!(!StoragePath::is_unix_path_str("C:\\"));
        assert!(!StoragePath::is_unix_path_str("relative"));
        assert!(!StoragePath::is_unix_path_str(""));
    }
}
