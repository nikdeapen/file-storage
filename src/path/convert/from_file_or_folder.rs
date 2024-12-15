use crate::{FilePath, FolderPath, StoragePath};

impl From<FilePath> for StoragePath {
    fn from(path: FilePath) -> Self {
        path.to_path()
    }
}

impl From<FolderPath> for StoragePath {
    fn from(path: FolderPath) -> Self {
        path.to_path()
    }
}
