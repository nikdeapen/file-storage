use crate::system::LocalPath;
use crate::Operation::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! List Files to Vec

    /// Lists the files from the folder and appends them to the `target` `Vec`.
    ///
    /// Returns `Ok(file_count)`.
    ///
    /// # Note
    /// Only the appended files will be sorted, not the entire `target`.
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

    /// Lists the files to the `target` `Vec`.
    ///
    /// Returns `Ok(file_count)`.
    pub fn list_files_to_vec_unsorted(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
        if let Some(local) = LocalPath::from(self.path()) {
            return local.list_files_to_vec_unsorted(target);
        }

        #[cfg(feature = "r2")]
        if let Some(r2) = crate::R2Path::from(self.path()) {
            return r2.list_files_to_vec(target);
        }

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
