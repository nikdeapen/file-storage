use std::io;

use crate::{Error, StoragePath};

impl StoragePath {
    //! Parse

    /// Parses the `path`.
    pub fn parse<S>(path: S) -> Result<Self, io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        let path: &str = path.as_ref();

        if StoragePath::is_unix_path_str(path) {
            return Ok(unsafe { StoragePath::new(path, 1, '/') });
        } else if StoragePath::is_windows_path_str(path) {
            // todo -- refactor windows paths
            // Windows paths are rather poorly designed. I am not sure if I want to normalize the
            // path case and file-separators here or keep it as is and handle de-duplication and
            // other logic in other places. I am also not sure if I should have equality be
            // case-insensitive. I don't use this 🗑🔥 of an OS so I will address this later.
            let file_separator: char = path.as_bytes()[2] as char;
            return Ok(unsafe { StoragePath::new(path, 3, file_separator) });
        }

        #[cfg(feature = "r2")]
        if let Some(base_len) = crate::R2Path::base_len(path) {
            return Ok(unsafe { StoragePath::new(path, base_len, '/') });
        }

        Err(Error::unknown_file_system(path))
    }
}
