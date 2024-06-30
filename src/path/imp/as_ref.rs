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

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FilePath, FolderPath, Path};

    #[test]
    fn path_as_ref() {
        let path: Path = Path::unix_root().with_appended("path");
        assert_eq!(path.as_str(), "/path");
        assert_eq!(path.as_ref(), "/path");
    }

    #[test]
    fn file_as_ref() {
        let path: FilePath = Path::unix_root().make_file("path").unwrap();
        assert_eq!(path.as_str(), "/path");
        assert_eq!(path.as_ref(), "/path");
    }

    #[test]
    fn folder_as_ref() {
        let path: FolderPath = Path::unix_root().make_folder();
        assert_eq!(path.as_str(), "/");
        assert_eq!(path.as_ref(), "/");
    }
}
