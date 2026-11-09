use crate::{Error, FilePath, FolderPath, StoragePath};
use std::io;
use std::path::PathBuf;
use uuid::Uuid;

impl StoragePath {
    //! Temp

    /// Creates a temp path.
    ///
    /// # Note
    /// - The name is a random UUID in the system's temp folder.
    /// - Nothing is created or checked on disk.
    pub fn temp() -> Result<Self, io::Error> {
        let path: PathBuf = std::env::temp_dir().join(Uuid::new_v4().to_string());
        match path.to_str() {
            Some(path) => Self::parse(path),
            None => Err(Error::path_not_utf8(&path)),
        }
    }
}

impl FilePath {
    //! Temp

    /// Creates a temp file path.
    ///
    /// # Note
    /// - The file is not created. See `StoragePath::temp`.
    pub fn temp() -> Result<Self, io::Error> {
        StoragePath::temp()?.to_file().map_err(Into::into)
    }
}

impl FolderPath {
    //! Temp

    /// Creates a temp folder path.
    ///
    /// # Note
    /// - The folder is not created. See `StoragePath::temp`.
    pub fn temp() -> Result<Self, io::Error> {
        Ok(StoragePath::temp()?.make_folder())
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, FolderPath, StoragePath};
    use std::path::Path;

    #[test]
    fn temp_is_new_and_unique() {
        let first: FilePath = FilePath::temp().unwrap();
        let second: FilePath = FilePath::temp().unwrap();
        assert_ne!(first, second);
        assert!(!Path::new(first.path().path()).exists());
    }

    #[test]
    fn temp_folder() {
        let folder: FolderPath = FolderPath::temp().unwrap();
        let path: &StoragePath = folder.path();
        let temp_dir: String = std::env::temp_dir().into_os_string().into_string().unwrap();
        assert!(path.is_folder());
        assert!(path.path().starts_with(&temp_dir));
    }
}
