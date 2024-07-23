use std::io::ErrorKind::Other;

use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Temp

    /// Creates a local temp path.
    pub fn temp() -> Result<Self, std::io::Error> {
        let path: tempfile::NamedTempFile = tempfile::NamedTempFile::new()?;
        let path: &str = path
            .path()
            .to_str()
            .ok_or(std::io::Error::new(Other, "temp path not UTF-8"))?;
        Self::parse(path)
    }
}

impl FilePath {
    //! Temp

    /// Creates a local temp file.
    pub fn temp() -> Result<Self, std::io::Error> {
        Ok(Path::temp()?
            .make_file("temp.file")
            .ok_or(std::io::Error::new(
                Other,
                "for some reason 'e' is a file separator in this file system ¯\\_(ツ)_/¯",
            ))?)
    }
}

impl FolderPath {
    //! Temp

    /// Creates a local temp folder.
    pub fn temp() -> Result<Self, std::io::Error> {
        Ok(Path::temp()?.make_folder())
    }
}
