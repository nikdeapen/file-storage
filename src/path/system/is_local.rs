use crate::StoragePath;

impl StoragePath {
    //! Is Local

    /// Checks if the path is a local path on this host.
    pub fn is_local_path(&self) -> bool {
        Self::is_local_path_str(self.path())
    }

    /// Checks if the `path` is a local path on this host.
    ///
    /// # Note
    /// - Windows paths are local only on Windows. Unix paths are local on every other host.
    /// - On Unix, `C:\a.txt` would otherwise name a file in the current directory.
    /// - It only checks how the path starts. `parse` can still reject it.
    pub fn is_local_path_str<S>(path: S) -> bool
    where
        S: AsRef<str>,
    {
        let path: &str = path.as_ref();
        if cfg!(windows) {
            Self::is_windows_path_str(path)
        } else {
            Self::is_unix_path_str(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn is_local_path_str() {
        let is_local = |path: &str| -> bool { StoragePath::is_local_path_str(path) };
        let on_windows: bool = cfg!(windows);

        assert_eq!(is_local("/unix/path"), !on_windows);
        assert_eq!(is_local("C:\\windows\\path"), on_windows);
        assert_eq!(is_local("\\\\server\\share"), on_windows);

        assert!(!is_local("https://example.com/bucket/"));
        assert!(!is_local("relative"));
        assert!(!is_local(""));
    }
}
