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
            return Ok(Self::parse_windows(path.into()));
        }

        #[cfg(feature = "r2")]
        if let Some(base_len) = crate::R2Path::base_len(s) {
            return Ok(unsafe { StoragePath::new(path, base_len, '/') });
        }

        Err(Error::unknown_file_system(s))
    }

    /// Parses a Windows path, normalizing file separators to `\`.
    fn parse_windows(mut path: String) -> Self {
        // Safety: replacing `/` (1 byte) with `\` (1 byte) preserves UTF-8 validity.
        unsafe {
            for byte in path.as_bytes_mut() {
                if *byte == b'/' {
                    *byte = b'\\';
                }
            }
        }
        let base_len: usize = Self::windows_base_len(&path);
        unsafe { StoragePath::new(path, base_len, '\\') }
    }

    /// Computes the base length for a normalized Windows path.
    ///
    /// Expects all file separators to already be normalized to `\`.
    fn windows_base_len(path: &str) -> usize {
        let bytes: &[u8] = path.as_bytes();

        // Drive letter path (C:\): base is the first 3 bytes.
        if bytes.len() >= 3
            && bytes[0].is_ascii_alphabetic()
            && bytes[1] == b':'
            && bytes[2] == b'\\'
        {
            return 3;
        }

        // UNC path (\\server\share\): base includes up to and including the share separator.
        if bytes.len() >= 3 && bytes[0] == b'\\' && bytes[1] == b'\\' {
            if let Some(server_end) = path[2..].find('\\') {
                let share_start: usize = 2 + server_end + 1;
                if share_start < path.len() {
                    if let Some(share_end) = path[share_start..].find('\\') {
                        return share_start + share_end + 1;
                    }
                }
            }
        }

        // Incomplete or unrecognized structure: use the entire path as the base.
        path.len()
    }
}
