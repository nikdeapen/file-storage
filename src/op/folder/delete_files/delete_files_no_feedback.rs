use crate::{Error, FolderPath};

impl FolderPath {
    //! Delete Files No Feedback

    /// Deletes the files in the folder.
    ///
    /// Returns `Ok(())`.
    pub fn delete_files_no_feedback(&self) -> Result<(), Error> {
        for file in &self.list_files_as_vec_unsorted()? {
            file.delete_if_exists()?;
        }
        Ok(())
    }
}
