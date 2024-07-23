use std::ffi::OsStr;

use crate::{FilePath, FolderPath, Path};

impl Path {
    //! AsRef

    /// Gets the path as a string.
    pub fn as_str(&self) -> &str {
        self.path()
    }
}

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        self.path().as_ref()
    }
}

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
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
