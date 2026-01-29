use crate::{Error, FilePath, FolderPath, StoragePath};
use std::io;
use tempfile::NamedTempFile;

impl StoragePath {
    //! Temp

    /// Creates a temp path.
    pub fn temp() -> Result<StoragePath, io::Error> {
        let named: NamedTempFile = NamedTempFile::new()?;
        if let Some(named) = named.path().to_str() {
            Ok(StoragePath::parse(named)?)
        } else {
            Err(Error::path_not_utf8(named.path()))
        }
    }
}

impl FilePath {
    //! Temp

    /// Creates a temp file path. (does not create the file)
    pub fn temp() -> Result<FilePath, io::Error> {
        StoragePath::temp()?
            .make_file("temp.folder")
            .map_err(|_| io::Error::other("¯\\_(ツ)_/¯"))
    }
}

impl FolderPath {
    //! Temp

    /// Creates a temp folder path. (does not create the folder)
    pub fn temp() -> Result<FolderPath, io::Error> {
        Ok(StoragePath::temp()?.make_folder())
    }
}
