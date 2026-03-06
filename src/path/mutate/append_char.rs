use crate::StoragePath;

impl StoragePath {
    //! Append Char

    /// Appends the char.
    pub fn append_char(&mut self, c: char) {
        unsafe { self.path_mut() }.push(c);
    }

    /// Appends the char.
    pub fn with_appended_char(mut self, c: char) -> Self {
        self.append_char(c);
        self
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
}
