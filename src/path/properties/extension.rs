use crate::FilePath;

impl FilePath {
    //! Extension

    /// Gets the file extension (the portion after the last `.` in the file name).
    ///
    /// Returns `None` if there is no `.` in the file name.
    pub fn extension(&self) -> Option<&str> {
        let name: &str = self.file_name();
        name.rfind('.').map(|pos| &name[pos + 1..])
    }
}

#[cfg(test)]
mod tests {
    use crate::FolderPath;
    use std::error::Error;

    #[test]
    fn extension() -> Result<(), Box<dyn Error>> {
        let root: FolderPath = FolderPath::unix_root();

        let file = root.clone().make_file("file.txt")?;
        assert_eq!(file.extension(), Some("txt"));

        let file = root.clone().make_file("archive.tar.gz")?;
        assert_eq!(file.extension(), Some("gz"));

        let file = root.clone().make_file("no_extension")?;
        assert_eq!(file.extension(), None);

        let file = root.clone().make_file(".gitignore")?;
        assert_eq!(file.extension(), Some("gitignore"));

        let file = root.clone().make_file("trailing.")?;
        assert_eq!(file.extension(), Some(""));

        Ok(())
    }
}
