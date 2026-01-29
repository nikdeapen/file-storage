use crate::{FilePath, FolderPath, StoragePath};

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

impl FilePath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        StoragePath::is_local_path_str(self.as_str())
    }
}

impl FolderPath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        StoragePath::is_local_path_str(self.as_str())
    }
}
