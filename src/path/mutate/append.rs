use crate::{FilePath, FolderPath, StoragePath};

impl StoragePath {
    //! Append

    /// Appends the `string`.
    pub fn append<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        unsafe { self.path_mut() }.push_str(string.as_ref());
    }

    /// Appends the `string`.
    pub fn with_appended<S>(mut self, string: S) -> Self
    where
        S: AsRef<str>,
    {
        self.append(string);
        self
    }

    /// Clones the path and appends the `string`.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but with a single allocation.
    pub fn clone_append<S>(&self, string: S) -> Self
    where
        S: AsRef<str>,
    {
        let s: &str = string.as_ref();
        self.clone_with_extra_capacity(s.len()).with_appended(s)
    }
}

impl FilePath {
    //! Append

    /// Appends the `string`.
    pub fn with_appended<S>(self, string: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(string)
    }

    /// Clones the path and appends the `string`.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but with a single allocation.
    pub fn clone_append<S>(&self, string: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.path().clone_append(string)
    }
}

impl FolderPath {
    //! Append

    /// Appends the `string`.
    pub fn with_appended<S>(self, string: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.to_path().with_appended(string)
    }

    /// Clones the path and appends the `string`.
    ///
    /// The result is the same as `path.clone().with_appended(s)` but with a single allocation.
    pub fn clone_append<S>(&self, string: S) -> StoragePath
    where
        S: AsRef<str>,
    {
        self.path().clone_append(string)
    }
}
