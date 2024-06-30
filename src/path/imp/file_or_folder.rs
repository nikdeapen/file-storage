use crate::path::Path;

impl Path {
    //! File or Folder

    /// Checks if the path is a file path.
    ///
    /// If the path is not a file path it is a folder path.
    pub fn is_file(&self) -> bool {
        !self.is_folder()
    }

    /// Checks if the path is a folder path.
    ///
    /// If the path is not a folder path it is a file path.
    pub fn is_folder(&self) -> bool {
        self.extension().is_empty() || self.path().ends_with(self.file_separator())
    }
}

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::Path;

    #[test]
    fn is_file_or_folder() {
        let path: Path = Path::unix_root().with_appended("the/path");
        assert!(path.is_file());
        assert!(!path.is_folder());

        let path: Path = path.with_appended("/");
        assert!(!path.is_file());
        assert!(path.is_folder());
    }
}
