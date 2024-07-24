use crate::{FilePath, Path};

impl FilePath {
    //! File Name

    /// Gets the file name.
    pub fn file_name(&self) -> &str {
        let extension: &str = self.path().extension();
        let fs: char = self.path().file_separator();
        if let Some(lfs_index) = extension.rfind(|c| c == fs) {
            &extension[(lfs_index + fs.len_utf8())..]
        } else {
            extension
        }
    }

    /// Gets the file name split from the extension.
    pub fn file_name_and_extension(&self) -> (&str, &str) {
        let file_name: &str = self.file_name();
        if let Some(dot) = file_name.as_bytes().iter().position(|c| *c == b'.') {
            file_name.split_at(dot)
        } else {
            (file_name, "")
        }
    }

    /// Removes the extension.
    ///
    /// # Examples
    /// /the/file.txt           `Ok(/the/file)`
    /// /the/file.one.two       `Ok(/the/file.one)`
    /// /the/file.              `Ok(/the/file)`
    /// /the/file               `Err(no extension)`
    /// /the/.txt               `Err(empty file name)`
    pub fn without_extension(self) -> Result<Self, (Self, &'static str)> {
        let file_name: &str = self.file_name();
        if let Some(last_dot) = file_name.as_bytes().iter().rposition(|c| *c == b'.') {
            if last_dot == 0 {
                Err((self, "empty file name"))
            } else {
                let new_len: usize = self.len() - file_name.len() + last_dot;
                let path: Path = self.to_path();
                let path: Path = unsafe { path.truncated(new_len) };
                let path: FilePath = path.to_file().unwrap();
                Ok(path)
            }
        } else {
            Err((self, "no extension"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{FilePath, Path};

    #[test]
    fn without_extension() {
        let test_cases: &[(&str, Option<&str>)] = &[
            ("/the/file.txt", Some("/the/file")),
            ("/the/file.one.two", Some("/the/file.one")),
            ("/the/file.", Some("/the/file")),
            ("/the/file", None),
            ("/the/.txt", None),
        ];
        for (path, expected) in test_cases {
            let path: FilePath = Path::parse(*path).unwrap().to_file().unwrap();
            let result: Option<FilePath> = path.without_extension().ok();
            if let Some(result) = result {
                assert_eq!(result.as_str(), expected.unwrap());
            } else {
                assert_eq!(*expected, None);
            }
        }
    }
}
