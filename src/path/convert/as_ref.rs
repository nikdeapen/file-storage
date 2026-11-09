use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! AsRef

    /// Gets the path as a string.
    pub fn as_str(&self) -> &str {
        self.path()
    }
}

impl AsRef<str> for StoragePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<StoragePath> for StoragePath {
    fn as_ref(&self) -> &StoragePath {
        self
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<StoragePath> for FilePath {
    fn as_ref(&self) -> &StoragePath {
        self.path()
    }
}

impl AsRef<FilePath> for FilePath {
    fn as_ref(&self) -> &FilePath {
        self
    }
}

impl AsRef<str> for FolderPath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<StoragePath> for FolderPath {
    fn as_ref(&self) -> &StoragePath {
        self.path()
    }
}

impl AsRef<FolderPath> for FolderPath {
    fn as_ref(&self) -> &FolderPath {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath, StoragePath};

    fn as_str<P>(path: &P) -> &str
    where
        P: AsRef<str>,
    {
        path.as_ref()
    }

    fn as_storage_path<P>(path: &P) -> &StoragePath
    where
        P: AsRef<StoragePath>,
    {
        path.as_ref()
    }

    #[test]
    fn storage_path() {
        let path: StoragePath = StoragePath::parse("/folder/file.txt").unwrap();
        assert_eq!(path.as_str(), "/folder/file.txt");
        assert_eq!(as_str(&path), "/folder/file.txt");
        assert_eq!(as_storage_path(&path), &path);
    }

    #[test]
    fn file_path() {
        let file: FilePath = StoragePath::parse("/folder/file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        assert_eq!(as_str(&file), "/folder/file.txt");
        assert_eq!(as_storage_path(&file), file.path());
        let same: &FilePath = file.as_ref();
        assert_eq!(same, &file);
    }

    #[test]
    fn folder_path() {
        let folder: FolderPath = StoragePath::parse("/folder/").unwrap().to_folder().unwrap();
        assert_eq!(as_str(&folder), "/folder/");
        assert_eq!(as_storage_path(&folder), folder.path());
        let same: &FolderPath = folder.as_ref();
        assert_eq!(same, &folder);
    }
}
