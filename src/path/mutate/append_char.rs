use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Append Char

    /// Appends the char.
    pub fn append_char(&mut self, c: char) {
        unsafe { self.path_mut_unchecked() }.push(c);
    }

    /// Appends the char.
    pub fn with_appended_char(mut self, c: char) -> Self {
        self.append_char(c);
        self
    }
}

impl FilePath {
    //! Append Char

    /// Appends the char.
    pub fn with_appended_char(self, c: char) -> StoragePath {
        self.to_path().with_appended_char(c)
    }
}

impl FolderPath {
    //! Append Char

    /// Appends the char.
    pub fn with_appended_char(self, c: char) -> StoragePath {
        self.to_path().with_appended_char(c)
    }
}