use crate::{FilePath, FolderPath};

impl FilePath {
    //! Rebase

    /// Rebase the file in the current folder to the target folder.
    ///
    /// # Example
    /// `self`          /the/file.txt
    /// `current`       /the/
    /// `target`        /other/
    /// `result`        /other/file.txt
    ///
    /// Returns `None` when `self` is not in `current`.
    ///
    /// This may also return `None` if the file and target folder have different file separators
    /// and the file ends with the target folder's file separator.
    pub fn rebase(&self, current: &FolderPath, target: &FolderPath) -> Option<FilePath> {
        if self.path().path().starts_with(current.path().path()) {
            let extension: &str = &self.path().path()[current.path().len()..];
            target
                .path()
                .clone_with_extra_capacity(extension.len())
                .with_appended(extension)
                .to_file()
        } else {
            None
        }
    }
}
