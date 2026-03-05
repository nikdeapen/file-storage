use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Is System

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        Self::is_windows_path_str(self.path())
    }

    /// Checks if the `path` is a Windows path.
    ///
    /// Matches drive letter paths (`C:\` or `C:/`) and UNC paths (`\\server\...`).
    pub fn is_windows_path_str<S>(path: S) -> bool
    where
        S: AsRef<str>,
    {
        let path: &str = path.as_ref();
        let bytes: &[u8] = path.as_bytes();
        if bytes.len() < 3 {
            return false;
        }
        let is_drive_letter: bool = bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && (bytes[2] == b'\\' || bytes[2] == b'/');
        let is_unc: bool =
            bytes[0] == b'\\' && bytes[1] == b'\\' && bytes[2] != b'\\';
        is_drive_letter || is_unc
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

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_windows_path_str() {
        // Drive letter paths.
        assert!(StoragePath::is_windows_path_str("C:\\"));
        assert!(StoragePath::is_windows_path_str("C:/"));
        assert!(StoragePath::is_windows_path_str("D:\\folder\\file"));

        // UNC paths.
        assert!(StoragePath::is_windows_path_str("\\\\server\\share"));
        assert!(StoragePath::is_windows_path_str("\\\\s\\share\\file"));

        // Not Windows paths.
        assert!(!StoragePath::is_windows_path_str("/unix/path"));
        assert!(!StoragePath::is_windows_path_str("C:"));
        assert!(!StoragePath::is_windows_path_str("\\\\\\bad"));
        assert!(!StoragePath::is_windows_path_str("ab"));
    }
}
