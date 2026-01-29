use crate::{Error, StoragePath};
use std::io;

impl StoragePath {
    //! Parse

    /// Parses the `path`.
    pub fn parse<S>(path: S) -> Result<Self, io::Error>
    where
        S: AsRef<str> + Into<String>,
    {
        let s: &str = path.as_ref();

        if StoragePath::is_unix_path_str(s) {
            return Ok(unsafe { StoragePath::new(path, 1, '/') });
        } else if StoragePath::is_windows_path_str(s) {
            // todo -- refactor windows paths
            // Windows paths are rather poorly designed. I am not sure if I want to normalize the
            // path case and file-separators here or keep it as is and handle de-duplication and
            // other logic in other places. I am also not sure if I should have equality be
            // case-insensitive. I don't use this 🗑🔥 of an OS so I will address this later.
            // todo -- this logic is very interdependent and should be refactored as well
            let file_separator: char = s.as_bytes()[2] as char;
            return Ok(unsafe { StoragePath::new(path, 3, file_separator) });
        }

        Err(Error::unknown_file_system(s))
    }
}
