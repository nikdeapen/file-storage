use crate::{Error, FilePath, FolderPath, StoragePath};
use std::io;
use tempfile::NamedTempFile;

impl StoragePath {
    //! Temp

    /// Creates a temp path.
    pub fn temp() -> Result<StoragePath, io::Error> {
        let temp_file: NamedTempFile = NamedTempFile::new()?;
        if let Some(path) = temp_file.path().to_str() {
            Ok(StoragePath::parse(path)?)
        } else {
            Err(Error::path_not_utf8(temp_file.path()))
        }
    }
}

impl FilePath {
    //! Temp

    /// Creates a temp file path. (does not create the file)
    pub fn temp() -> Result<FilePath, io::Error> {
        StoragePath::temp()?.to_file().map_err(Into::into)
    }
}

impl FolderPath {
    //! Temp

    /// Creates a temp folder path. (does not create the folder)
    pub fn temp() -> Result<FolderPath, io::Error> {
        Ok(StoragePath::temp()?.make_folder())
    }
}
