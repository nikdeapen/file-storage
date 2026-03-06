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
    use crate::{FolderPath, StoragePath};
    use std::error::Error;

    #[test]
    fn file_name() -> Result<(), Box<dyn Error>> {
        let root: FolderPath = FolderPath::unix_root();

        let file = root.clone().make_file("file.txt")?;
        assert_eq!(file.file_name(), "file.txt");

        let folder: FolderPath = StoragePath::parse("/foo/bar/")?.to_folder()?;
        let file = folder.make_file("data.json")?;
        assert_eq!(file.file_name(), "data.json");

        let file = root.make_file("solo")?;
        assert_eq!(file.file_name(), "solo");

        Ok(())
    }
}
