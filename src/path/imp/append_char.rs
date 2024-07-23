use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Append Char

    /// Appends the char.
    pub fn append_char(&mut self, c: char) {
        self.path_mut().push(c);
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
    pub fn with_appended_char(self, c: char) -> Path {
        self.to_path().with_appended_char(c)
    }
}

impl FolderPath {
    //! Append Char

    /// Appends the char.
    pub fn with_appended_char(self, c: char) -> Path {
        self.to_path().with_appended_char(c)
    }
}
