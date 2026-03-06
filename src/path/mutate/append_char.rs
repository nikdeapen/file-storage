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
    use crate::{FolderPath, StoragePath};
    use std::error::Error;

    #[test]
    fn append() -> Result<(), Box<dyn Error>> {
        let path: StoragePath = StoragePath::unix_root();
        let result: StoragePath = path.with_appended_char('c');
        assert_eq!(result.as_str(), "/c");

        let file = FolderPath::unix_root().make_file("file")?;
        let result: StoragePath = file.to_path().with_appended_char('c');
        assert_eq!(result.as_str(), "/filec");

        let folder: FolderPath = FolderPath::unix_root();
        let result: StoragePath = folder.to_path().with_appended_char('c');
        assert_eq!(result.as_str(), "/c");

        Ok(())
    }
}
