use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Is System

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        Self::is_windows_path_str(self.path())
    }

    /// Checks if the `path` is a Windows path.
    pub fn is_windows_path_str<S>(path: S) -> bool
    where
        S: AsRef<str>,
    {
        let path: &str = path.as_ref();
        if path.len() < 3 {
            false
        } else {
            path.as_bytes()[0].is_ascii_alphabetic()
                && path.as_bytes()[1] == b':'
                && (path.as_bytes()[2] == b'\\' || path.as_bytes()[2] == b'/')
        }
    }
}

impl FilePath {
    //! Is System

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        StoragePath::is_windows_path_str(self.as_str())
    }
}

impl FolderPath {
    //! Is System

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        StoragePath::is_windows_path_str(self.as_str())
    }
}
