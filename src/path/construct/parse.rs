use std::io;

use crate::{is_unix_path, is_windows_path, Error, StoragePath};

impl StoragePath {
    //! Parse

    /// Parses the path for the known file systems.
    pub fn parse<S>(s: S) -> Result<Self, io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        let s_ref: &str = s.as_ref();

        if is_unix_path(s_ref) {
            Ok(unsafe { StoragePath::new_unchecked(s, 1, '/') })
        } else if is_windows_path(s_ref) {
            let file_separator: char = s_ref.as_bytes()[2] as char;
            Ok(unsafe { StoragePath::new_unchecked(s, 3, file_separator) })
        } else {
            Err(Error::unsupported_file_system(s_ref))
        }
    }
}
