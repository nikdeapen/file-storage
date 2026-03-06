use crate::{Error, FilePath, FolderPath, Operation, Reason, StoragePath};

impl FolderPath {
    //! Make File

    /// Appends the `file_name` to the folder path, creating a file path.
    ///
    /// Returns `Err` if the `file_name` is empty or ends with the file-separator.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, Error> {
        if file_name.is_empty() || file_name.ends_with(self.file_separator()) {
            Err(Error::new(self, Operation::ModifyPath, Reason::InvalidPath))
        } else {
            let path: StoragePath = self.to_path().with_appended(file_name);
            Ok(unsafe { FilePath::new(path) })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::FolderPath;
    use std::error::Error;

    #[test]
    fn make_file() -> Result<(), Box<dyn Error>> {
        let folder: FolderPath = FolderPath::unix_root();
        let file = folder.make_file("folder.txt")?;
        assert_eq!(file.as_str(), "/folder.txt");

        Ok(())
    }
}
