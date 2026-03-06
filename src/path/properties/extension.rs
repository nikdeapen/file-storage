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
    use crate::{FilePath, StoragePath};

    fn file(path: &str) -> FilePath {
        StoragePath::parse(path).unwrap().to_file().unwrap()
    }

    #[test]
    fn extension() {
        let f: FilePath = file("/file.txt");
        assert_eq!(f.extension(), Some("txt"));
        assert_eq!(f.dot_extension(), Some(".txt"));

        let f: FilePath = file("/archive.tar.gz");
        assert_eq!(f.extension(), Some("gz"));
        assert_eq!(f.dot_extension(), Some(".gz"));

        let f: FilePath = file("/no_extension");
        assert_eq!(f.extension(), None);
        assert_eq!(f.dot_extension(), None);

        let f: FilePath = file("/.gitignore");
        assert_eq!(f.extension(), Some("gitignore"));
        assert_eq!(f.dot_extension(), Some(".gitignore"));

        let f: FilePath = file("/trailing.");
        assert_eq!(f.extension(), Some(""));
        assert_eq!(f.dot_extension(), Some("."));
    }
}
