use std::io;
use std::io::ErrorKind::InvalidData;

use tempfile::NamedTempFile;

use crate::{Error, FilePath, FolderPath, StoragePath};

impl FilePath {
    //! Temp File

    /// Creates a new local temp file.
    pub fn temp() -> Result<Self, io::Error> {
        let temp: NamedTempFile = NamedTempFile::new()?;
        if let Some(temp) = temp.path().to_str() {
            let path: StoragePath = StoragePath::parse(temp)?;
            let file: Self = path.make_file("temp.file").ok_or_else(|| {
                io::Error::new(InvalidData, "'e' is a file separator ¯\\_(ツ)_/¯")
            })?;
            Ok(file)
        } else {
            Err(Error::path_not_utf8(temp.path().to_path_buf()))
        }
    }
}

impl FolderPath {
    //! Temp Folder

    /// Creates a new local temp folder.
    pub fn temp() -> Result<Self, io::Error> {
        let temp: NamedTempFile = NamedTempFile::new()?;
        if let Some(temp) = temp.path().to_str() {
            let mut s: String = String::with_capacity(temp.len() + 1);
            s.push_str(temp);
            let path: StoragePath = StoragePath::parse(s)?;
            Ok(path.make_folder())
        } else {
            Err(Error::path_not_utf8(temp.path().to_path_buf()))
        }
    }
}
