/// A file or folder path.
#[derive(Clone, Debug)]
pub struct Path {
    path: String,
    base_len: usize,
    file_separator: char,
}

impl Path {
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

impl Path {
    //! Properties

    /// Gets the path string.
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    /// Gets the mutable path string.
    pub fn path_mut(&mut self) -> &mut String {
        &mut self.path
    }

    /// Gets the path length. (in bytes)
    pub fn len(&self) -> usize {
        self.path.len()
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
    use crate::path::Path;

    #[test]
    fn properties() {
        let mut path: Path = unsafe { Path::new_unchecked("/the/path", 1, '/') };
        assert_eq!(path.path(), "/the/path");
        assert_eq!(path.path_mut(), "/the/path");
        assert_eq!(path.len(), "/the/path".len());
        assert_eq!(path.base_len(), 1);
        assert_eq!(path.file_separator(), '/');
    }
}
