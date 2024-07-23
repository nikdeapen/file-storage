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
mod tests {
    use crate::Path;

    #[test]
    fn is_file_or_folder() {
        let path: Path = unsafe { Path::new_unchecked("/file", 1, '/') };
        assert!(path.is_file());
        assert!(!path.is_folder());

        let path: Path = unsafe { Path::new_unchecked("/folder/", 1, '/') };
        assert!(!path.is_file());
        assert!(path.is_folder());
    }
}
