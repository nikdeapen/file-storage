use crate::FilePath;

impl FilePath {
    //! File Name

    /// Gets the file name (the last segment of the path).
    pub fn file_name(&self) -> &str {
        let relative: &str = self.relative_path();
        if let Some(pos) = relative.rfind(self.file_separator()) {
            &relative[pos + self.file_separator().len_utf8()..]
        } else {
            relative
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, StoragePath};

    #[test]
    fn file_name() {
        let file: FilePath = StoragePath::parse("/file.txt").unwrap().to_file().unwrap();
        assert_eq!(file.file_name(), "file.txt");

        let file: FilePath = StoragePath::parse("/foo/bar/data.json")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.file_name(), "data.json");

        let file: FilePath = StoragePath::parse("/solo").unwrap().to_file().unwrap();
        assert_eq!(file.file_name(), "solo");
    }

    #[test]
    fn file_name_windows() {
        let file: FilePath = StoragePath::parse("C:\\folder\\file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.file_name(), "file.txt");

        let file: FilePath = StoragePath::parse("C:\\file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.file_name(), "file.txt");
    }

    #[test]
    fn file_name_unix_keeps_backslashes() {
        let file: FilePath = StoragePath::parse("/folder/a\\b")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.file_name(), "a\\b");
    }

    #[cfg(feature = "r2")]
    #[test]
    fn file_name_r2() {
        for path in [
            "https://account.r2.cloudflarestorage.com/bucket/file.txt",
            "https://account.r2.cloudflarestorage.com/bucket/folder/file.txt",
        ] {
            let file: FilePath = StoragePath::parse(path).unwrap().to_file().unwrap();
            assert_eq!(file.file_name(), "file.txt", "{path}");
        }
    }
}
