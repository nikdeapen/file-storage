use crate::{Error, FolderPath};

impl FolderPath {
    //! Delete Files

    /// Deletes the files in the folder.
    ///
    /// Returns `Ok(())`.
    pub fn delete_files(&self) -> Result<(), Error> {
        for file in &self.list_files_as_vec_unsorted()? {
            file.delete()?;
        }
        Ok(())
    }
}
