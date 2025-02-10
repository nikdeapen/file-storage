use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Append

    /// Appends the string.
    pub fn append<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        unsafe { self.path_mut() }.push_str(s.as_ref());
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
        self.clone_with_extra_capacity(s.len()).with_appended(s)
    }
}

impl FilePath {
    //! Append

    /// Appends the string.
    pub fn with_appended<S>(self, s: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(s)
    }

    /// Clones the path and appends the string.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but minimizes allocation.
    pub fn clone_append<S>(&self, s: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.path().clone_append(s)
    }
}

impl FolderPath {
    //! Append

    /// Appends the string.
    pub fn with_appended<S>(self, s: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(s)
    }

    /// Clones the path and appends the string.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but minimizes allocation.
    pub fn clone_append<S>(&self, s: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.path().clone_append(s)
    }
}
