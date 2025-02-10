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
            let path: StoragePath = StoragePath::parse(named)?;
            Ok(path)
        } else {
            Err(Error::path_not_utf8(path))
        }
    }
}

impl FilePath {
    //! Temp

    /// Creates a temp file.
    pub fn temp() -> Result<FilePath, io::Error> {
        let path: StoragePath = StoragePath::temp()?;
        if path.is_file() {
            Ok(path.make_file("temp.file").ok_or_else(|| {
                io::Error::new(
                    Other,
                    "`e` is a file separator on this file system ¯\\_(ツ)_/¯.",
                )
            })?)
        } else {
            Err(Error::unknown_file_system(path.as_ref()))
        }
    }
}

impl FolderPath {
    //! Temp

    /// Creates a temp folder.
    pub fn temp() -> Result<FolderPath, io::Error> {
        let path: StoragePath = StoragePath::temp()?;
        if path.is_file() {
            Ok(path.make_folder())
        } else {
            Err(Error::unknown_file_system(path.as_ref()))
        }
    }
}
