use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! List Files to Vec

    /// Appends the listed files to the `target` vec sorted in lexicographical order.
    ///
    /// Note: Only the appended files will be sorted, not the entire `target`.
    ///
    /// Returns `Ok(file_count)`.
    pub fn list_files_to_vec(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.list_files_to_vec(target);
        }

        let original_len: usize = target.len();
        let file_count: usize = self.list_files_to_vec_unsorted(target)?;
        let slice: &mut [FilePath] = &mut target.as_mut_slice()[original_len..];
        slice.sort_unstable();
        Ok(file_count)
    }
}
