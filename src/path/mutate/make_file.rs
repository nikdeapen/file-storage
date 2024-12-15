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
    pub fn make_file(self, file_name: &str) -> Option<FilePath> {
        if self.is_file() {
            self.to_file()
        } else {
            self.with_appended(file_name).to_file()
        }
    }
}

impl FolderPath {
    //! Make File

    /// Makes the folder path a file path.
    ///
    /// Returns `None` if the `file_name` is empty or ends with the file-separator.
    pub fn make_file(self, file_name: &str) -> Option<FilePath> {
        self.to_path().make_file(file_name)
    }
}
