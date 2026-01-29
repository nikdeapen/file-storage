use crate::{FilePath, FolderPath, StoragePath};

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
        path.as_ref().starts_with("/")
    }
}

impl FilePath {
    //! Is System

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        StoragePath::is_unix_path_str(self.as_str())
    }
}

impl FolderPath {
    //! Is System

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        StoragePath::is_unix_path_str(self.as_str())
    }
}
