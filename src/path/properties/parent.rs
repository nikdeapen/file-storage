use crate::{FolderPath, StoragePath};

impl StoragePath {
    //! Parent

    /// Gets the parent folder path.
    ///
    /// Returns `None` if the path is the base path.
    pub fn parent(&self) -> Option<FolderPath> {
        self.clone().to_parent()
    }

    /// Converts to the parent folder path.
    ///
    /// Returns `None` if the path is the base path.
    pub fn to_parent(mut self) -> Option<FolderPath> {
        let relative: &str = self.relative_path();
        if relative.is_empty() {
            None
        } else {
            let no_trailing_fs: &str = if relative.ends_with(self.file_separator()) {
                &relative[..relative.len() - self.file_separator().len_utf8()]
            } else {
                relative
            };
            let new_len: usize = match no_trailing_fs.rfind(self.file_separator()) {
                Some(pos) => pos + self.file_separator().len_utf8(),
                None => 0,
            };
            let new_len: usize = self.base_len() + new_len;
            unsafe {
                self.truncate(new_len);
                Some(FolderPath::new(self))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::StoragePath;

    #[test]
    fn parent_of_file() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path/file.txt", 1, '/') };
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_str(), "/the/path/");
    }

    #[test]
    fn parent_of_file_in_root() {
        let path: StoragePath = unsafe { StoragePath::new("/file.txt", 1, '/') };
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_str(), "/");
    }

    #[test]
    fn parent_of_folder() {
        let path: StoragePath = unsafe { StoragePath::new("/the/path/", 1, '/') };
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_str(), "/the/");
    }

    #[test]
    fn parent_of_folder_in_root() {
        let path: StoragePath = unsafe { StoragePath::new("/folder/", 1, '/') };
        let parent = path.parent().unwrap();
        assert_eq!(parent.as_str(), "/");
    }

    #[test]
    fn parent_of_root() {
        let path: StoragePath = unsafe { StoragePath::new("/", 1, '/') };
        assert!(path.parent().is_none());
    }
}
