use std::io;
use std::io::ErrorKind::Other;
use std::path::Path;

use tempfile::NamedTempFile;

use crate::{Error, FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Temp

    /// Creates a temp path.
    pub fn temp() -> Result<StoragePath, io::Error> {
        let named: NamedTempFile = NamedTempFile::new()?;
        let path: &Path = named.path();
        if let Some(named) = named.path().to_str() {
            Ok(StoragePath::parse(named)?)
        } else {
            Err(Error::path_not_utf8(path))
        }
    }
}

impl FilePath {
    //! Temp

    /// Creates a temp file path. (does not create the file)
    pub fn temp() -> Result<FilePath, io::Error> {
        let path: StoragePath = StoragePath::temp()?;
        if path.is_file() {
            path.make_file("temp.file")
                .map_err(|_| io::Error::new(Other, "¯\\_(ツ)_/¯"))
        } else {
            Err(Error::unknown_file_system(path.as_ref()))
        }
    }
}

impl FolderPath {
    //! Temp

    /// Creates a temp folder path. (does not create the folder)
    pub fn temp() -> Result<FolderPath, io::Error> {
        let path: StoragePath = StoragePath::temp()?;
        if path.is_file() {
            Ok(path.make_folder())
        } else {
            Err(Error::unknown_file_system(path.as_ref()))
        }
    }
}
