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

#[cfg(test)]
#[cfg(feature = "local")]
mod tests {
    use crate::{FilePath, FolderPath, Path};

    #[test]
    fn append() {
        let path: Path = Path::unix_root();
        assert_eq!(path.path(), "/");

        let mut path: Path = path.with_appended("one");
        assert_eq!(path.path(), "/one");

        path.append("/two");
        assert_eq!(path.path(), "/one/two");

        let file: FilePath = path.to_file().unwrap();
        let path: Path = file.with_appended("/three/");
        assert_eq!(path.path(), "/one/two/three/");

        let folder: FolderPath = path.to_folder().unwrap();
        let path: Path = folder.with_appended("four");
        assert_eq!(path.path(), "/one/two/three/four");
    }
}
