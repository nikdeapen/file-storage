use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! List Files as Vec Unsorted

    /// Lists the files as a `Vec<FilePath>`.
    ///
    /// Returns `Ok(files)`.
    pub fn list_files_as_vec_unsorted(&self) -> Result<Vec<FilePath>, Error> {
        let mut target: Vec<FilePath> = Vec::new();
        self.list_files_to_vec_unsorted(&mut target)?;
        Ok(target)
    }
}