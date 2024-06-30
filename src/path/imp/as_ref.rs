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

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FilePath, FolderPath, Path};

    #[test]
    fn path_as_ref() {
        let path: Path = Path::unix_root().with_appended("path");
        assert_eq!(path.as_str(), "/path");

        let result: &str = path.as_ref();
        assert_eq!(result, "/path");

        let result: &std::path::Path = path.as_ref();
        assert_eq!(result.as_os_str().to_str().unwrap(), "/path");
    }

    #[test]
    fn file_as_ref() {
        let path: FilePath = Path::unix_root().make_file("path").unwrap();
        assert_eq!(path.as_str(), "/path");

        let result: &str = path.as_ref();
        assert_eq!(result, "/path");

        let result: &std::path::Path = path.as_ref();
        assert_eq!(result.as_os_str().to_str().unwrap(), "/path");
    }

    #[test]
    fn folder_as_ref() {
        let path: FolderPath = Path::unix_root().make_folder();
        assert_eq!(path.as_str(), "/");

        let result: &str = path.as_ref();
        assert_eq!(result, "/");

        let result: &std::path::Path = path.as_ref();
        assert_eq!(result.as_os_str().to_str().unwrap(), "/");
    }
}
