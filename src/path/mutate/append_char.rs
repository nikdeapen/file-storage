use crate::StoragePath;

impl StoragePath {
    //! Append Char

    /// Appends the char.
    ///
    /// # Note
    /// - On a Windows path, `/` is appended as `\`.
    pub fn append_char(&mut self, c: char) {
        let c: char = Self::normal_char(c, self.file_separator());
        unsafe { self.path_mut() }.push(c);
    }

    /// Appends the char.
    pub fn with_appended_char(mut self, c: char) -> Self {
        self.append_char(c);
        self
    }

    /// Gets the `c` as it is stored in a path with the `file_separator`.
    ///
    /// # Note
    /// - Windows paths store `/` as `\`. Parsing and appending both apply the rule here.
    pub(in crate::path) fn normal_char(c: char, file_separator: char) -> char {
        if c == '/' && file_separator == '\\' {
            '\\'
        } else {
            c
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn append() {
        let path: StoragePath = StoragePath::unix_root();
        let result: StoragePath = path.with_appended_char('c');
        assert_eq!(result.as_str(), "/c");

        let file: StoragePath = StoragePath::unix_root().with_appended("file");
        let result: StoragePath = file.with_appended_char('c');
        assert_eq!(result.as_str(), "/filec");

        let folder_path: StoragePath = StoragePath::unix_root().with_appended("folder/");
        let result: StoragePath = folder_path.with_appended_char('c');
        assert_eq!(result.as_str(), "/folder/c");
    }

    #[test]
    fn append_windows_separator() {
        let path: StoragePath = StoragePath::parse("C:\\folder").unwrap();
        let result: StoragePath = path.with_appended_char('/');
        assert_eq!(result.as_str(), "C:\\folder\\");
        assert!(result.is_folder());
    }

    #[test]
    fn append_unix_backslash() {
        let result: StoragePath = StoragePath::unix_root().with_appended_char('\\');
        assert_eq!(result.as_str(), "/\\");
    }
}
