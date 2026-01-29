use crate::{FilePath, StoragePath};

impl From<FilePath> for StoragePath {
    fn from(path: FilePath) -> Self {
        path.to_path()
    }
}
