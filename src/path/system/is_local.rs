use crate::StoragePath;

impl StoragePath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        Self::is_local_path_str(self.path())
    }

    /// Checks if the `path` is a local path.
    pub fn is_local_path_str<S>(path: S) -> bool
    where
        S: AsRef<str>,
    {
        let path: &str = path.as_ref();
        StoragePath::is_unix_path_str(path) || StoragePath::is_windows_path_str(path)
    }
}
