use std::io::ErrorKind::Other;

use tempfile::NamedTempFile;

use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Roots

    /// Gets the unix root path.
    #[cfg(feature = "local")]
    pub fn unix_root() -> Self {
        unsafe { Self::new_unchecked("/", 1, '/') }
    }
}

impl Path {
    //! Parse

    /// Parses the path for known path structures.
    ///
    /// # Errors
    /// Returns an error if the path is not part of a known file system.
    pub fn parse<S>(path: S) -> Result<Self, std::io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        #[cfg(feature = "local")]
        if crate::local::is_unix_path(path.as_ref()) {
            return Ok(unsafe { Self::new_unchecked(path, 1, '/') });
        }

        #[cfg(feature = "local")]
        if crate::local::is_windows_path(path.as_ref()) {
            let b: &[u8] = path.as_ref().as_bytes();
            debug_assert!(b.len() >= 3);
            debug_assert!(b[2] == b'\\' || b[2] == b'/');
            let file_separator: char = b[2] as char;
            return Ok(unsafe { Self::new_unchecked(path, 3, file_separator) });
        }

        Err(std::io::Error::new(
            Other,
            format!("unknown file system for path: {}", path.as_ref()),
        ))
    }
}

impl Path {
    //! Temp

    /// Creates a local temp path.
    #[cfg(feature = "local")]
    pub fn temp() -> Result<Self, std::io::Error> {
        let path: NamedTempFile = NamedTempFile::new()?;
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
    #[cfg(feature = "local")]
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
    //! Roots

    /// Gets the unix root path.
    #[cfg(feature = "local")]
    pub fn unix_root() -> Self {
        Path::unix_root().to_folder().unwrap()
    }
}

impl FolderPath {
    //! Temp

    /// Creates a local temp folder.
    #[cfg(feature = "local")]
    pub fn temp() -> Result<Self, std::io::Error> {
        Ok(Path::temp()?.make_folder())
    }
}

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FolderPath, Path};

    #[test]
    fn unix_root() {
        assert_eq!(Path::unix_root().path(), "/");
        assert_eq!(FolderPath::unix_root().path().path(), "/");
    }
}
