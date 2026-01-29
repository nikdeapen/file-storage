use crate::{FilePath, FolderPath, StoragePath};
use std::ffi::OsStr;

impl StoragePath {
    //! AsRef

    /// Gets the path as a string.
    pub fn as_str(&self) -> &str {
        self.path()
    }
}

impl AsRef<str> for StoragePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<std::path::Path> for StoragePath {
    fn as_ref(&self) -> &std::path::Path {
        self.path().as_ref()
    }
}

impl AsRef<OsStr> for StoragePath {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl AsRef<StoragePath> for StoragePath {
    fn as_ref(&self) -> &StoragePath {
        self
    }
}

impl FilePath {
    //! AsRef

    /// Gets the path as a string.
    pub fn as_str(&self) -> &str {
        self.path().path()
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<std::path::Path> for FilePath {
    fn as_ref(&self) -> &std::path::Path {
        self.path().as_ref()
    }
}

impl AsRef<OsStr> for FilePath {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        self
    }
}

impl FolderPath {
    //! AsRef

    /// Gets the path as a string.
    pub fn as_str(&self) -> &str {
        self.path().path()
    }
}

impl AsRef<str> for FolderPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<std::path::Path> for FolderPath {
    fn as_ref(&self) -> &std::path::Path {
        self.path().as_ref()
    }
}

impl AsRef<OsStr> for FolderPath {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl AsRef<FolderPath> for FolderPath {
    fn as_ref(&self) -> &FolderPath {
        self
    }
}
