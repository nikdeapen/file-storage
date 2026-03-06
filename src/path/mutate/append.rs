use crate::StoragePath;

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

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn append() {
        let path: StoragePath = StoragePath::unix_root();
        let result: StoragePath = path.with_appended("folder.txt");
        assert_eq!(result.as_str(), "/folder.txt");

        let file: StoragePath = StoragePath::unix_root().with_appended("folder");
        let result: StoragePath = file.with_appended(".txt");
        assert_eq!(result.as_str(), "/folder.txt");

        let folder_path: StoragePath = StoragePath::unix_root().with_appended("folder/");
        let result: StoragePath = folder_path.with_appended("file.txt");
        assert_eq!(result.as_str(), "/folder/file.txt");
    }
}
