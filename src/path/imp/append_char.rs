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

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FilePath, FolderPath, Path};

    #[test]
    fn append() {
        let path: Path = Path::unix_root();
        assert_eq!(path.path(), "/");

        let mut path: Path = path.with_appended_char('1');
        assert_eq!(path.path(), "/1");

        path.append_char('/');
        path.append_char('2');
        assert_eq!(path.path(), "/1/2");

        let file: FilePath = path.to_file().unwrap();
        let path: Path = file.with_appended_char('/');
        assert_eq!(path.path(), "/1/2/");

        let folder: FolderPath = path.to_folder().unwrap();
        let path: Path = folder.with_appended_char('3');
        assert_eq!(path.path(), "/1/2/3");
    }
}
