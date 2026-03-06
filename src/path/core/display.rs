use crate::{FilePath, FolderPath, StoragePath};
use std::fmt::{Display, Formatter};

impl Display for StoragePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.path())
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Display for FolderPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
