/// A file or folder path.
///
/// # Base Path and Relative Path
/// Storage paths are divided into a base path and a relative path. The base path is effectively
/// immutable. The relative path is a sequence of segments divided by file separators.
///
/// ```text
/// Unix Example:
/// Path:           /the/path
/// Base Path:      /
/// Relative Path:  the/path
///
/// Windows Example:
/// Path:           C:\The\Path
/// Base Path:      C:\
/// Relative Path:  The\Path
///
/// Cloudflare R2 Example:
/// Path:           https://account.r2.cloudflarestorage.com/bucket/the/path
/// Base Path:      https://account.r2.cloudflarestorage.com/bucket/
/// Relative Path:  the/path
/// ```
///
/// # Files and Folders
/// A storage path can be a file path or a folder path. These are mutually exclusive and
/// exhaustive. A storage path is a folder path if it is equal to its base path or ends with a
/// file separator. Otherwise it is a file path. This invariant can be broken with `unsafe`.
///
/// # File Separators
/// - Windows paths use `\`. Unix and R2 paths use `/`.
/// - A Windows path never holds `/`. Parsing and appending turn each `/` into `\`. A path built
///   by appending then equals the same path parsed in one piece.
/// - Unix and R2 paths keep `\` as is. It is a valid character in their names.
/// - Like the file and folder rule, this can be broken with `unsafe`.
///
/// # Comparison
/// - Paths compare by their exact text.
/// - Letter case matters on Windows too. `C:\Foo` and `C:\foo` are not equal.
#[must_use]
#[derive(Clone, Debug)]
pub struct StoragePath {
    path: String,
    base_len: usize,
    file_separator: char,
}

impl StoragePath {
    //! Construction

    /// Creates the [StoragePath].
    ///
    /// # Safety
    /// - The `base_len` must be on a char boundary in the `path`.
    /// - The `base_len` must cover the whole base path.
    /// - A Windows path must hold no `/`.
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
    /// - The first `base_len` bytes must not change.
    /// - A Windows path must not gain a `/`.
    pub unsafe fn path_mut(&mut self) -> &mut String {
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
        let mut path: StoragePath = unsafe { StoragePath::new("/the/path", 1, '/') };
        assert_eq!(path.path(), "/the/path");
        unsafe {
            assert_eq!(path.path_mut(), "/the/path");
        }
        assert_eq!(path.base_len(), 1);
        assert_eq!(path.file_separator(), '/');
    }
}
