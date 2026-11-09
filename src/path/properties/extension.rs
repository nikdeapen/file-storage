use crate::FilePath;

impl FilePath {
    //! Extension

    /// Gets the file extension.
    ///
    /// # Note
    /// - It is the part of the file name after the last `.`. It is `None` if there is no `.`.
    /// - A leading `.` counts. The file `.gitignore` has the extension `gitignore`.
    pub fn extension(&self) -> Option<&str> {
        self.dot_extension().map(|e| &e[1..])
    }

    /// Gets the file extension with the leading `.`.
    ///
    /// # Note
    /// - It is `None` if there is no `.` in the file name. See `extension`.
    pub fn dot_extension(&self) -> Option<&str> {
        let file_name: &str = self.file_name();
        file_name.rfind('.').map(|pos| &file_name[pos..])
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
