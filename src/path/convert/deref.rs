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
