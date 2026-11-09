use crate::StoragePath;

impl StoragePath {
    //! Is Windows

    /// Checks if the path is a Windows path.
    pub fn is_windows_path(&self) -> bool {
        Self::is_windows_path_str(self.path())
    }

    /// Checks if the `path` is a Windows path.
    ///
    /// # Note
    /// - A drive letter path starts like `C:\` or `C:/`.
    /// - A UNC path starts like `\\server`.
    /// - It only checks how the path starts. `parse` can still reject it.
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
        let is_unc: bool = bytes[0] == b'\\' && bytes[1] == b'\\' && bytes[2] != b'\\';
        is_drive_letter || is_unc
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
        assert!(StoragePath::is_windows_path_str("c:\\"));

        // UNC paths.
        assert!(StoragePath::is_windows_path_str("\\\\server\\share"));
        assert!(StoragePath::is_windows_path_str("\\\\s\\share\\file"));

        // Not Windows paths.
        assert!(!StoragePath::is_windows_path_str("/unix/path"));
        assert!(!StoragePath::is_windows_path_str("C:"));
        assert!(!StoragePath::is_windows_path_str("\\\\\\bad"));
        assert!(!StoragePath::is_windows_path_str("ab"));
    }

    #[test]
    fn is_windows_path_str_does_not_validate() {
        for path in ["\\\\server", "\\\\?\\C:\\folder", "\\\\/server"] {
            assert!(StoragePath::is_windows_path_str(path), "{path}");
            assert!(StoragePath::parse(path).is_err(), "{path}");
        }
    }
}
