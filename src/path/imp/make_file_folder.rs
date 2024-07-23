use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Make File & Make Folder

    /// Makes the path a file path.
    ///
    /// If the path is a file path it will be returned unchanged. if the path is a folder path the
    /// `file_name` will be appended. If the resulting path is a file path it will be returned,
    /// otherwise this will return `None`.
    pub fn make_file<S>(self, file_name: S) -> Option<FilePath>
    where
        S: AsRef<str>,
    {
        if self.is_file() {
            self.to_file()
        } else {
            let file_name: &str = file_name.as_ref();
            if file_name.is_empty() || file_name.ends_with(self.file_separator()) {
                None
            } else {
                self.with_appended(file_name).to_file()
            }
        }
    }

    /// Makes the path a folder path.
    ///
    /// If the path is a folder path the folder path will be returned unaltered. if the path is a
    /// file path a file separator will be appended and the resulting folder path will be returned.
    pub fn make_folder(self) -> FolderPath {
        if self.is_folder() {
            self.to_folder().unwrap()
        } else {
            let file_separator: char = self.file_separator();
            self.with_appended_char(file_separator).to_folder().unwrap()
        }
    }
}

impl FilePath {
    //! Make Folder

    /// Makes the file path a folder path by appending a file separator.
    pub fn make_folder(self) -> FolderPath {
        self.to_path().make_folder()
    }
}

impl FolderPath {
    //! Make File

    /// Makes the folder path a file path by appending the file name.
    ///
    /// Returns `None` if the file name is empty or ends with a file separator.
    pub fn make_file<S>(self, file_name: S) -> Option<FilePath>
    where
        S: AsRef<str>,
    {
        self.to_path().make_file(file_name)
    }
}
