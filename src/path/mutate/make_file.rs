use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Make File

    /// Makes the path a file path.
    ///
    /// If the path is a file the file will be returned.
    ///
    /// If the path is a folder the `file_name` will be appended then:
    ///     1. If the path is a file the file will be returned.
    ///     2. If the `file_name` is empty or ends with the file-separator it will be `None`.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, StoragePath> {
        if self.is_file() {
            self.to_file()
        } else {
            let path: StoragePath = self.with_appended(file_name);
            if path.is_file() {
                path.to_file()
            } else {
                let original_len: usize = path.len() - file_name.len();
                Err(unsafe { path.truncated(original_len) })
            }
        }
    }
}

impl FolderPath {
    //! Make File

    /// Makes the folder path a file path.
    ///
    /// Returns `None` if the `file_name` is empty or ends with the file-separator.
    pub fn make_file(self, file_name: &str) -> Result<FilePath, FolderPath> {
        self.to_path()
            .make_file(file_name)
            .map_err(|path| path.to_folder().unwrap())
    }
}
