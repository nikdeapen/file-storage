use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Make Folder

    /// Clones the path and makes the path a folder.
    ///
    /// # Note
    /// - A folder is returned as is.
    /// - A file gets a file separator appended.
    pub fn clone_make_folder(&self) -> FolderPath {
        let path: StoragePath = if self.is_folder() {
            self.clone()
        } else {
            self.clone_with_extra_capacity(self.file_separator().len_utf8())
                .with_appended_char(self.file_separator())
        };
        unsafe { FolderPath::new(path) }
    }

    /// Makes the storage path a folder.
    ///
    /// # Note
    /// - A folder is returned as is.
    /// - A file gets a file separator appended.
    pub fn make_folder(self) -> FolderPath {
        let path: StoragePath = if self.is_folder() {
            self
        } else {
            let separator: char = self.file_separator();
            self.with_appended_char(separator)
        };
        unsafe { FolderPath::new(path) }
    }
}

impl FilePath {
    //! Make Folder

    /// Makes the file a folder by appending a file separator.
    pub fn make_folder(self) -> FolderPath {
        self.to_path().make_folder()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath, StoragePath};

    #[test]
    fn make_folder() {
        let path: StoragePath = StoragePath::unix_root();

        let path: StoragePath = path.make_folder().to_path();
        assert_eq!(path.as_str(), "/");

        let path: StoragePath = path.with_appended("folder.txt").make_folder().to_path();
        assert_eq!(path.as_str(), "/folder.txt/");
    }

    #[test]
    fn make_folder_windows() {
        let folder: FolderPath = StoragePath::parse("C:\\folder").unwrap().make_folder();
        assert_eq!(folder.as_str(), "C:\\folder\\");

        let root: FolderPath = StoragePath::parse("C:\\").unwrap().make_folder();
        assert_eq!(root.as_str(), "C:\\");
    }

    #[test]
    fn make_folder_keeps_folder() {
        for path in ["/folder/", "C:\\folder\\"] {
            let path: StoragePath = StoragePath::parse(path).unwrap();
            assert_eq!(path.clone_make_folder().as_str(), path.as_str());
            assert_eq!(path.clone().make_folder().as_str(), path.as_str());
        }
    }

    #[test]
    fn clone_make_folder() {
        let path: StoragePath = StoragePath::parse("C:\\folder\\file").unwrap();
        let folder: FolderPath = path.clone_make_folder();
        assert_eq!(folder.as_str(), "C:\\folder\\file\\");
        assert_eq!(path.as_str(), "C:\\folder\\file");
    }

    #[test]
    fn file_make_folder() {
        let file: FilePath = StoragePath::parse("/folder/file")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(file.make_folder().as_str(), "/folder/file/");
    }
}
