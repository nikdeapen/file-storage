use crate::{FilePath, FolderPath, Path};

impl FolderPath {
    //! Make File

    /// Converts the folder into its parent file path.
    ///
    /// Example:
    /// /the/folder/path/    ->    /the/folder/path
    ///
    /// Returns `None` if the folder path is the base path.
    pub fn to_parent_file_path(self) -> Option<Path> {
        let path: Path = self.to_path();
        if path.len() == path.base_len() {
            None
        } else {
            let new_len: usize = path.len() - path.file_separator().len_utf8();
            let path: Path = unsafe { path.truncated(new_len) };
            Some(path)
        }
    }

    /// Converts the folder into its parent file.
    ///
    /// Example:
    /// /the/folder/path/    ->    /the/folder/path
    ///
    /// Returns `None` if the folder path is the base path or the final folder name is empty.
    pub fn to_parent_file(self) -> Option<FilePath> {
        if let Some(path) = self.to_parent_file_path() {
            path.to_file()
        } else {
            None
        }
    }
}
