use crate::{Error, FilePath, FolderPath, Operation, Reason, StoragePath};

impl StoragePath {
    //! Make File

    /// Makes the path a file path.
    ///
    /// If the path is a file, the file will be returned.
    ///
    /// If the path is a folder, the `file_name` will be appended then:
    ///     1. If the path is a file, the file will be returned.
    ///     2. If the `file_name` is empty or ends with the file-separator, it will be `None`.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, Error> {
        if self.is_file() {
            self.to_file()
        } else {
            let path: StoragePath = self.with_appended(file_name);
            if path.is_file() {
                path.to_file()
            } else {
                let original_len: usize = path.len() - file_name.len();
                let path: StoragePath = unsafe { path.truncated(original_len) };
                Err(Error::new(path, Operation::ModifyPath, Reason::Other))
            }
        }
    }
}

impl FolderPath {
    //! Make File

    /// Makes the folder path a file path.
    ///
    /// Returns `None` if the `file_name` is empty or ends with the file-separator.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, Error> {
        self.to_path().make_file(file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;
    use std::error::Error;

    #[test]
    fn make_file() -> Result<(), Box<dyn Error>> {
        let path: StoragePath = StoragePath::unix_root();

        let path: StoragePath = path.make_file("folder.txt")?.to_path();
        assert_eq!(path.as_str(), "/folder.txt");

        let path: StoragePath = path.make_file("ignored")?.to_path();
        assert_eq!(path.as_str(), "/folder.txt");

        Ok(())
    }
}
