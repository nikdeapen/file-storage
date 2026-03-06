use crate::{FolderPath, StoragePath};

impl StoragePath {
    //! Parent

    /// Gets the parent folder.
    ///
    /// Returns `None` if the path is at the base (root, drive, or bucket).
    pub fn parent(&self) -> Option<FolderPath> {
        let relative: &str = self.relative_path();
        if relative.is_empty() {
            return None;
        }

        let sep_len: usize = self.file_separator().len_utf8();

        // For folder paths, skip the trailing separator before searching.
        let search_end: usize = if self.is_folder() {
            relative.len() - sep_len
        } else {
            relative.len()
        };
        let search: &str = &relative[..search_end];

        let new_len: usize = if let Some(pos) = search.rfind(self.file_separator()) {
            self.base_len() + pos + sep_len
        } else {
            self.base_len()
        };

        let parent: StoragePath = unsafe { self.clone().truncated(new_len) };
        Some(parent.make_folder())
    }
}

#[cfg(test)]
mod tests {
    use crate::{FolderPath, StoragePath};
    use std::error::Error;

    #[test]
    fn parent_of_file() -> Result<(), Box<dyn Error>> {
        let file = StoragePath::parse("/foo/bar/file.txt")?.to_file()?;
        assert_eq!(file.parent().unwrap().as_str(), "/foo/bar/");
        Ok(())
    }

    #[test]
    fn parent_of_file_at_root() -> Result<(), Box<dyn Error>> {
        let file = StoragePath::parse("/file.txt")?.to_file()?;
        assert_eq!(file.parent().unwrap().as_str(), "/");
        Ok(())
    }

    #[test]
    fn parent_of_folder() -> Result<(), Box<dyn Error>> {
        let folder = StoragePath::parse("/foo/bar/")?.to_folder()?;
        assert_eq!(folder.parent().unwrap().as_str(), "/foo/");
        Ok(())
    }

    #[test]
    fn parent_of_folder_at_root() -> Result<(), Box<dyn Error>> {
        let folder = StoragePath::parse("/foo/")?.to_folder()?;
        assert_eq!(folder.parent().unwrap().as_str(), "/");
        Ok(())
    }

    #[test]
    fn parent_of_root() {
        let root: FolderPath = FolderPath::unix_root();
        assert!(root.parent().is_none());
    }

    #[test]
    fn parent_of_windows_file() -> Result<(), Box<dyn Error>> {
        let file = StoragePath::parse("C:\\foo\\file.txt")?.to_file()?;
        assert_eq!(file.parent().unwrap().as_str(), "C:\\foo\\");
        Ok(())
    }

    #[test]
    fn parent_of_windows_root_file() -> Result<(), Box<dyn Error>> {
        let file = StoragePath::parse("C:\\file.txt")?.to_file()?;
        assert_eq!(file.parent().unwrap().as_str(), "C:\\");
        Ok(())
    }
}
