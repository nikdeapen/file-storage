use crate::{FilePath, FolderPath, StoragePath};

impl From<FilePath> for StoragePath {
    fn from(file_path: FilePath) -> Self {
        file_path.to_path()
    }
}

impl From<&FilePath> for StoragePath {
    fn from(file_path: &FilePath) -> Self {
        file_path.clone().to_path()
    }
}

impl From<FolderPath> for StoragePath {
    fn from(folder_path: FolderPath) -> Self {
        folder_path.to_path()
    }
}

impl From<&FolderPath> for StoragePath {
    fn from(folder_path: &FolderPath) -> Self {
        folder_path.clone().to_path()
    }
}
