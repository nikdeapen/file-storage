/// A file or folder path.
///
/// # Base & Extension
/// Paths are divided into a base and an extension. The base is immutable and the extension is a
/// sequence of segments divided by file-separators.
///
/// Unix Example:
/// Path:           /the/path
/// Base:           /
/// Extension:      the/path
///
/// Windows Example:
/// Path:           C:\The\Path
/// Base:           C:\
/// Extension:      The\Path
///
/// URL Example:
/// Path:           https://example.com/the/path
/// Base:           https://example.com/
/// Extension:      the/path
///
/// # Files & Folders
/// A path can be a file path or a folder path. These are mutually exclusive and exhaustive. A path
/// is a folder if it is equal to its base or ends with a file-separator, otherwise it is a file.
#[derive(Clone, Debug)]
pub struct StoragePath {
    path: String,
    base_len: usize,
    file_separator: char,
}

impl StoragePath {
    //! Construction

    /// Creates a new path.
    ///
    /// # Unsafe
    /// The `base_len` must be a valid char separator in `path`.
    pub unsafe fn new_unchecked<S>(path: S, base_len: usize, file_separator: char) -> Self
    where
        S: Into<String>,
    {
        let path: String = path.into();

        debug_assert!(path.is_char_boundary(base_len));

        Self {
            path,
            base_len,
            file_separator,
        }
    }
}

impl StoragePath {
    //! Properties

    /// Gets the path string.
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Gets the mutable path string.
    ///
    /// # Unsafe
    /// The path string must remain valid for the path.
    pub unsafe fn path_mut_unchecked(&mut self) -> &mut String {
        &mut self.path
    }

    /// Gets the base length. (in bytes)
    pub fn base_len(&self) -> usize {
        self.base_len
    }

    /// Gets the file separator.
    pub fn file_separator(&self) -> char {
        self.file_separator
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn properties() {
        let s: &str = "/the/path";
        let mut path: StoragePath = unsafe { StoragePath::new_unchecked(s, 1, '/') };

        assert_eq!(path.path(), s);
        assert_eq!(unsafe { path.path_mut_unchecked() }, s);
        assert_eq!(path.base_len(), 1);
        assert_eq!(path.file_separator(), '/');
    }
}
