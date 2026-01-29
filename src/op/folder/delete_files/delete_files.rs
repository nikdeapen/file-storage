use crate::{Error, FolderPath, ListFilesOp};

impl FolderPath {
    //! Delete Files

    /// Deletes the files in the folder.
    pub fn delete_files(&self) -> Result<(), Error> {
        // todo -- this can be more performant on most systems
        let files: ListFilesOp = self.list_files()?;
        for file in files {
            file?.delete()?;
        }
        Ok(())
    }
}
