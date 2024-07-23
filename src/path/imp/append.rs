use crate::{FilePath, FolderPath, Path};

impl Path {
    //! Append

    /// Appends the string.
    pub fn append<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        self.path_mut().push_str(s.as_ref());
    }

    /// Appends the string.
    pub fn with_appended<S>(mut self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        self.append(s);
        self
    }

    /// Clones the path and appends the string.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but minimizes allocation.
    pub fn clone_append<S>(&self, s: S) -> Self
    where
        S: AsRef<str>,
    {
        let s: &str = s.as_ref();
        let mut path: String = String::with_capacity(self.len() + s.len());
        path.push_str(self.path());
        path.push_str(s);
        unsafe { Self::new_unchecked(path, self.base_len(), self.file_separator()) }
    }
}

impl FilePath {
    //! Append

    /// Appends the string.
    pub fn with_appended<S>(self, s: S) -> Path
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(s)
    }
}

impl FolderPath {
    //! Append

    /// Appends the string.
    pub fn with_appended<S>(self, s: S) -> Path
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(s)
    }
}
