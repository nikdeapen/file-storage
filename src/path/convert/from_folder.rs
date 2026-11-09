use crate::{FolderPath, StoragePath};

impl From<FolderPath> for StoragePath {
    fn from(path: FolderPath) -> Self {
        path.to_path()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FolderPath, StoragePath};

    #[test]
    fn from_folder() {
        let folder: FolderPath = StoragePath::parse("/folder/").unwrap().to_folder().unwrap();
        let path: StoragePath = StoragePath::from(folder.clone());
        assert_eq!(&path, folder.path());
    }
}
