use crate::{FolderPath, StoragePath};

impl From<FolderPath> for StoragePath {
    fn from(path: FolderPath) -> Self {
        path.to_path()
    }
}
