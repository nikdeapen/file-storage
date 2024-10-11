use std::io;

use crate::{is_unix_path, is_windows_path, Error, StoragePath};

impl StoragePath {
    //! Parse

    /// Parses the path for the known file systems.
    pub fn parse<S>(path: S) -> Result<Self, io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        let path: &str = path.as_ref();

        if is_unix_path(path) {
            Ok(unsafe { StoragePath::new_unchecked(path, 1, '/') })
        } else if is_windows_path(path) {
            // todo -- refactor
            // Windows paths are weird. I am not sure if I want to normalize the path here or keep
            // it as is. I am also not sure if I should refactor equality to be case-insensitive.
            let file_separator: char = path.as_bytes()[2] as char;
            Ok(unsafe { StoragePath::new_unchecked(path, 3, file_separator) })
        } else {
            Err(Error::unsupported_file_system(path))
        }
    }
}
