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
    use crate::StoragePath;

    #[test]
    fn file_name() {
        let file = StoragePath::parse("/file.txt").unwrap().to_file().unwrap();
        assert_eq!(file.file_name(), "file.txt");

        let file = StoragePath::parse("/foo/bar/data.json").unwrap().to_file().unwrap();
        assert_eq!(file.file_name(), "data.json");

        let file = StoragePath::parse("/solo").unwrap().to_file().unwrap();
        assert_eq!(file.file_name(), "solo");
    }
}
