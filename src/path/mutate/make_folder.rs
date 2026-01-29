use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Make Folder

    /// Clones the path and makes the path a folder.
    ///
    /// Returns the path as is if it is already a folder, otherwise appends a file-separator.
    pub fn clone_make_folder(&self) -> FolderPath {
        if self.is_folder() {
            self.clone().to_folder().unwrap()
        } else {
            self.clone_with_extra_capacity(self.file_separator().len_utf8())
                .with_appended_char(self.file_separator())
                .to_folder()
                .unwrap()
        }
    }

    /// Makes the storage path a folder.
    ///
    /// Returns the path as is if it is already a folder, otherwise appends a file-separator.
    pub fn make_folder(self) -> FolderPath {
        if self.is_folder() {
            self.to_folder().unwrap()
        } else {
            let fs: char = self.file_separator();
            self.with_appended_char(fs).to_folder().unwrap()
        }
    }
}

impl FilePath {
    //! Make Folder

    /// Makes the file a folder by appending a file-separator.
    pub fn make_folder(self) -> FolderPath {
        let mut path: StoragePath = self.to_path();
        let file_separator: char = path.file_separator();
        unsafe { path.path_mut().push(file_separator) };
        path.to_folder().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn make_folder() {
        let path: StoragePath = StoragePath::unix_root();

        let path: StoragePath = path.make_folder().to_path();
        assert_eq!(path.as_str(), "/");

        let path: StoragePath = path.with_appended("folder.txt").make_folder().to_path();
        assert_eq!(path.as_str(), "/folder.txt/");
    }
}
