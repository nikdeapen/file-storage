use crate::Operation::ListFiles;
use crate::Reason::UnknownFileSystem;
use crate::{Error, FilePath, FolderPath, LocalPath};

impl FolderPath {
    //! List Files as Vec

    /// Lists the files in the folder as a `Vec<FilePath>` sorted in lexicographical order.
    ///
    /// Returns `Ok(sorted_files)`.
    pub fn list_files_as_vec(&self) -> Result<Vec<FilePath>, Error> {
        let mut vec: Vec<FilePath> = Vec::default();
        self.list_files_to_vec(&mut vec)?;
        Ok(vec)
    }

    /// Lists the files as a `Vec<FilePath>`.
    ///
    /// Returns `Ok(files)`.
    pub fn list_files_as_vec_unsorted(&self) -> Result<Vec<FilePath>, Error> {
        let mut target: Vec<FilePath> = Vec::new();
        self.list_files_to_vec_unsorted(&mut target)?;
        Ok(target)
    }

    /// Lists the files from the folder and appends them to the `target` `Vec`.
    ///
    /// Returns `Ok(file_count)`.
    ///
    /// # Note
    /// Only the appended files will be sorted, not the entire `target`.
    pub fn list_files_to_vec(&self, target: &mut Vec<FilePath>) -> Result<usize, Error> {
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

        Err(Error::new(self.clone(), ListFiles, UnknownFileSystem))
    }
}
