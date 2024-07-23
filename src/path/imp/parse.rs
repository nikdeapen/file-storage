use std::io::ErrorKind::Other;

use crate::local::is_unix_path;
use crate::Path;

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
        let s: &str = path.as_ref();

        if is_unix_path(s) {
            return Ok(unsafe { Path::new_unchecked(path, 1, '/') });
        }

        Err(std::io::Error::new(
            Other,
            format!("unknown file system for path: {}", path.as_ref()),
        ))
    }
}
