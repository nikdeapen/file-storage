use crate::StoragePath;

impl StoragePath {
    //! Append

    /// Appends the `string`.
    ///
    /// # Note
    /// - On a Windows path, each `/` is appended as `\`.
    pub fn append<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        let string: &str = string.as_ref();
        if self.file_separator() == '\\' {
            for c in string.chars() {
                self.append_char(c);
            }
        } else {
            unsafe { self.path_mut() }.push_str(string);
        }
    }

    /// Appends the `string`.
    pub fn with_appended<S>(mut self, string: S) -> Self
    where
        S: AsRef<str>,
    {
        self.append(string);
        self
    }

    /// Clones the path and appends the `string`.
    ///
    /// # Note
    /// - It allocates once. The result equals `path.clone().with_appended(string)`.
    pub fn clone_append<S>(&self, string: S) -> Self
    where
        S: AsRef<str>,
    {
        let s: &str = string.as_ref();
        self.clone_with_extra_capacity(s.len()).with_appended(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn append() {
        let path: StoragePath = StoragePath::unix_root();
        let result: StoragePath = path.with_appended("folder.txt");
        assert_eq!(result.as_str(), "/folder.txt");

        let file: StoragePath = StoragePath::unix_root().with_appended("folder");
        let result: StoragePath = file.with_appended(".txt");
        assert_eq!(result.as_str(), "/folder.txt");

        let folder_path: StoragePath = StoragePath::unix_root().with_appended("folder/");
        let result: StoragePath = folder_path.with_appended("file.txt");
        assert_eq!(result.as_str(), "/folder/file.txt");
    }

    #[test]
    fn append_windows_separators() {
        let folder: StoragePath = StoragePath::parse("C:\\folder\\").unwrap();

        let result: StoragePath = folder.clone().with_appended("sub/file.txt");
        assert_eq!(result.as_str(), "C:\\folder\\sub\\file.txt");
        let parsed: StoragePath = StoragePath::parse("C:\\folder\\sub/file.txt").unwrap();
        assert_eq!(result, parsed);

        let result: StoragePath = folder.clone_append("sub/");
        assert_eq!(result.as_str(), "C:\\folder\\sub\\");
        assert!(result.is_folder());
    }

    #[test]
    fn append_unix_backslashes() {
        let result: StoragePath = StoragePath::unix_root().with_appended("a\\b");
        assert_eq!(result.as_str(), "/a\\b");
    }
}
