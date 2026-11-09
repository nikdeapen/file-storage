use crate::{FilePath, FolderPath, StoragePath};
use std::ops::Deref;

impl Deref for FilePath {
    type Target = StoragePath;

    fn deref(&self) -> &StoragePath {
        self.path()
    }
}

impl Deref for FolderPath {
    type Target = StoragePath;

    fn deref(&self) -> &StoragePath {
        self.path()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath, StoragePath};

    #[test]
    fn file_derefs_to_storage_path() {
        let file: FilePath = StoragePath::parse("/folder/file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        let path: &StoragePath = &file;
        assert_eq!(path, file.path());
        assert_eq!(file.base_path(), "/");
        assert_eq!(file.relative_path(), "folder/file.txt");
    }

    #[test]
    fn folder_derefs_to_storage_path() {
        let folder: FolderPath = StoragePath::parse("/folder/").unwrap().to_folder().unwrap();
        let path: &StoragePath = &folder;
        assert_eq!(path, folder.path());
        assert_eq!(folder.base_path(), "/");
        assert_eq!(folder.relative_path(), "folder/");
    }
}
