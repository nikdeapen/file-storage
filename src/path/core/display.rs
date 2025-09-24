use crate::{FilePath, FolderPath, StoragePath};
use std::fmt::{Display, Formatter};

impl Display for StoragePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}

impl Display for FolderPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}
