use crate::{Error, FilePath, FolderPath};

impl FolderPath {
    //! Delete Files

    /// Deletes the files in the folder.
    ///
    /// Returns a `Vec` with the deleted file paths.
    pub fn delete_files(&self) -> Result<Vec<FilePath>, Error> {
        let files: Vec<FilePath> = self.list_files_as_vec()?;
        for file in &files {
            file.delete()?;
        }
        Ok(files)
    }
}
