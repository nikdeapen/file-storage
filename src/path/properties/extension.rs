use crate::FilePath;

impl FilePath {
    //! Extension

    /// Gets the file extension (the portion after the last `.` in the file name).
    ///
    /// Returns `None` if there is no `.` in the file name.
    pub fn extension(&self) -> Option<&str> {
        self.dot_extension().map(|e| &e[1..])
    }

    /// Gets the file extension with the leading `.`.
    ///
    /// Returns `None` if there is no `.` in the file name.
    pub fn dot_extension(&self) -> Option<&str> {
        self.file_name().rfind('.').map(|pos| &self.file_name()[pos..])
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath};
    use std::error::Error;

    #[test]
    fn extension() -> Result<(), Box<dyn Error>> {
        let root: FolderPath = FolderPath::unix_root();

        let file: FilePath = root.clone().make_file("file.txt")?;
        assert_eq!(file.extension(), Some("txt"));
        assert_eq!(file.dot_extension(), Some(".txt"));

        let file: FilePath = root.clone().make_file("archive.tar.gz")?;
        assert_eq!(file.extension(), Some("gz"));
        assert_eq!(file.dot_extension(), Some(".gz"));

        let file: FilePath = root.clone().make_file("no_extension")?;
        assert_eq!(file.extension(), None);
        assert_eq!(file.dot_extension(), None);

        let file: FilePath = root.clone().make_file(".gitignore")?;
        assert_eq!(file.extension(), Some("gitignore"));
        assert_eq!(file.dot_extension(), Some(".gitignore"));

        let file: FilePath = root.clone().make_file("trailing.")?;
        assert_eq!(file.extension(), Some(""));
        assert_eq!(file.dot_extension(), Some("."));

        Ok(())
    }
}
