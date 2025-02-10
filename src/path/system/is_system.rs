use crate::{is_local_path, is_unix_path, is_windows_path, FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        is_local_path(self)
    }

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        is_unix_path(self)
    }

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        is_windows_path(self)
    }
}

impl FilePath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        is_local_path(self)
    }

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        is_unix_path(self)
    }

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        is_windows_path(self)
    }
}

impl FolderPath {
    //! Is System

    /// Checks if the path is a local path.
    pub fn is_local_path(&self) -> bool {
        is_local_path(self)
    }

    /// Checks if the path is a Unix path.
    pub fn is_unix_path(&self) -> bool {
        is_unix_path(self)
    }

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        is_windows_path(self)
    }
}
