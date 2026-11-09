use crate::{FilePath, StoragePath};

impl From<FilePath> for StoragePath {
    fn from(path: FilePath) -> Self {
        path.to_path()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, StoragePath};

    #[test]
    fn from_file() {
        let file: FilePath = StoragePath::parse("/folder/file.txt")
            .unwrap()
            .to_file()
            .unwrap();
        let path: StoragePath = StoragePath::from(file.clone());
        assert_eq!(&path, file.path());
    }
}
