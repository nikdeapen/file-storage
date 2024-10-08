use crate::{FilePath, FolderPath, StoragePath};

/// Checks if the path is a local path.
pub fn is_local_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    is_unix_path(s.as_ref()) || is_windows_path(s.as_ref())
}

/// Checks if the path is a Unix path.
pub fn is_unix_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    s.as_ref().starts_with("/")
}

/// Checks if the path is a Windows path.
pub fn is_windows_path<S>(s: S) -> bool
where
    S: AsRef<str>,
{
    let s: &str = s.as_ref();
    if s.len() < 3 {
        false
    } else {
        s.as_bytes()[0].is_ascii_uppercase()
            && s.as_bytes()[1] == b':'
            && (s.as_bytes()[2] == b'\\' || s.as_bytes()[2] == b'/')
    }
}

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
