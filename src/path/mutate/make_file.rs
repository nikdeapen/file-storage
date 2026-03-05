use crate::{Error, FilePath, FolderPath, Operation, Reason, StoragePath};

impl FolderPath {
    //! Make File

    /// Appends the `file_name` to the folder path, creating a file path.
    ///
    /// Returns `Err` if the `file_name` is empty or ends with the file-separator.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, Error> {
        let path: StoragePath = self.to_path().with_appended(file_name);
        if path.is_file() {
            path.to_file()
        } else {
            let original_len: usize = path.len() - file_name.len();
            let path: StoragePath = unsafe { path.truncated(original_len) };
            Err(Error::new(path, Operation::ModifyPath, Reason::Other))
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
