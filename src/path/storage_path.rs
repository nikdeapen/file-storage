/// A file or folder path.
///
/// # Base & Extension
/// Storage paths are divided into a base and an extension. The base is effectively immutable, and
/// the extension is a sequence of segments divided by file-separators.
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
/// A storage path can be a file path or a folder path. These are mutually exclusive and
/// exhaustive. A storage path is a folder path if it is equal to its base or ends with a
/// file-separator, otherwise it is a file path. This invariant can be broken with `unsafe`.
#[derive(Clone, Debug)]
pub struct StoragePath {
    path: String,
    base_len: usize,
    file_separator: char,
}

impl StoragePath {
    //! Construction

    /// Creates a new storage path.
    ///
    /// # Safety
    /// The `base_len` must be a valid char boundary in the `path`.
    pub unsafe fn new<S>(path: S, base_len: usize, file_separator: char) -> Self
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
    /// # Safety
    /// The path string must remain valid for the path.
    pub unsafe fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    /// Gets the base length. (in bytes)
    pub fn base_len(&self) -> usize {
        self.base_len
    }

    /// Gets the file-separator.
    pub fn file_separator(&self) -> char {
        self.file_separator
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn properties() {
        let mut path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.path(), "/the/path");
        unsafe {
            assert_eq!(path.path_mut(), "/the/path");
        }
        assert_eq!(path.base_len(), 1);
        assert_eq!(path.file_separator(), '/');
    }
}
